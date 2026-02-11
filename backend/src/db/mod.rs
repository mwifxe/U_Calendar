use sqlx::PgPool;
use uuid::Uuid;

use crate::models::*;

pub async fn get_full_calendario(pool: &PgPool) -> Result<CalendarioResponse, sqlx::Error> {
    let periodos = sqlx::query_as::<_, PeriodoRow>("SELECT * FROM periodos ORDER BY orden")
        .persistent(false)
        .fetch_all(pool)
        .await?;

    let semanas = sqlx::query_as::<_, SemanaRow>("SELECT * FROM semanas ORDER BY orden")
        .persistent(false)
        .fetch_all(pool)
        .await?;

    let actividades = sqlx::query_as::<_, ActividadRow>("SELECT * FROM actividades ORDER BY orden")
        .persistent(false)
        .fetch_all(pool)
        .await?;

    let parciales = sqlx::query_as::<_, ParcialRow>("SELECT * FROM parciales ORDER BY orden")
        .persistent(false)
        .fetch_all(pool)
        .await?;

    let notas_ids: Vec<(Uuid,)> =
        sqlx::query_as("SELECT actividad_id FROM notas WHERE contenido != ''")
            .persistent(false)
            .fetch_all(pool)
            .await?;
    let notas_set: std::collections::HashSet<Uuid> =
        notas_ids.into_iter().map(|(id,)| id).collect();

    let response = CalendarioResponse {
        periodos: periodos
            .into_iter()
            .map(|p| {
                let periodo_semanas: Vec<SemanaResponse> = semanas
                    .iter()
                    .filter(|s| s.periodo_id == p.id)
                    .map(|s| {
                        let semana_actividades: Vec<ActividadResponse> = actividades
                            .iter()
                            .filter(|a| a.semana_id == s.id)
                            .map(|a| ActividadResponse {
                                id: a.id,
                                materia: a.materia.clone(),
                                titulo: a.titulo.clone(),
                                fecha: a.fecha.clone(),
                                porcentaje: a.porcentaje.clone(),
                                descripcion: a.descripcion.clone(),
                                clase: a.clase.clone(),
                                tiene_notas: notas_set.contains(&a.id),
                            })
                            .collect();

                        let semana_parciales: Option<Vec<String>> = if s.es_parcial {
                            Some(
                                parciales
                                    .iter()
                                    .filter(|pc| pc.semana_id == s.id)
                                    .map(|pc| pc.materia.clone())
                                    .collect(),
                            )
                        } else {
                            None
                        };

                        SemanaResponse {
                            id: s.id,
                            codigo: s.codigo.clone(),
                            titulo: s.titulo.clone(),
                            fecha: s.fecha.clone(),
                            es_parcial: s.es_parcial,
                            es_vacaciones: s.es_vacaciones,
                            nota_especial: s.nota_especial.clone(),
                            parciales: semana_parciales,
                            actividades: semana_actividades,
                        }
                    })
                    .collect();

                PeriodoResponse {
                    id: p.id,
                    numero: p.numero,
                    nombre: p.nombre,
                    info: p.info,
                    semanas: periodo_semanas,
                }
            })
            .collect(),
    };

    Ok(response)
}

pub async fn semana_exists(pool: &PgPool, semana_id: Uuid) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM semanas WHERE id = $1)")
        .persistent(false)
        .bind(semana_id)
        .fetch_one(pool)
        .await
}

pub async fn create_actividad(
    pool: &PgPool,
    semana_id: Uuid,
    input: &CreateActividad,
) -> Result<ActividadRow, sqlx::Error> {
    let (max_orden,): (i16,) = sqlx::query_as(
        "SELECT COALESCE(MAX(orden), 0)::smallint FROM actividades WHERE semana_id = $1",
    )
        .persistent(false)
        .bind(semana_id)
        .fetch_one(pool)
        .await?;
    let next_orden = max_orden + 1;

    let clase = input
        .clase
        .clone()
        .unwrap_or_else(|| get_clase_materia(&input.materia));

    sqlx::query_as::<_, ActividadRow>(
        r#"
        INSERT INTO actividades (semana_id, materia, titulo, fecha, porcentaje, descripcion, clase, orden)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING *
        "#,
    )
        .persistent(false)
        .bind(semana_id)
        .bind(&input.materia)
        .bind(&input.titulo)
        .bind(&input.fecha)
        .bind(&input.porcentaje)
        .bind(&input.descripcion)
        .bind(&clase)
        .bind(next_orden)
        .fetch_one(pool)
        .await
}

pub async fn update_actividad(
    pool: &PgPool,
    actividad_id: Uuid,
    input: &UpdateActividad,
) -> Result<Option<ActividadRow>, sqlx::Error> {
    let current = sqlx::query_as::<_, ActividadRow>("SELECT * FROM actividades WHERE id = $1")
        .persistent(false)
        .bind(actividad_id)
        .fetch_optional(pool)
        .await?;

    let Some(current) = current else {
        return Ok(None);
    };

    let materia = input.materia.as_deref().unwrap_or(&current.materia);
    let titulo = input.titulo.as_deref().unwrap_or(&current.titulo);
    let fecha = input.fecha.as_ref().or(current.fecha.as_ref());
    let porcentaje = input.porcentaje.as_ref().or(current.porcentaje.as_ref());
    let descripcion = input.descripcion.as_ref().or(current.descripcion.as_ref());
    let clase = input.clase.as_deref().unwrap_or_else(|| {
        if input.materia.is_some() {
            // Recalculate if materia changed
            ""
        } else {
            &current.clase
        }
    });
    let clase = if clase.is_empty() && input.materia.is_some() {
        get_clase_materia(materia)
    } else if clase.is_empty() {
        current.clase.clone()
    } else {
        clase.to_string()
    };

    let row = sqlx::query_as::<_, ActividadRow>(
        r#"
        UPDATE actividades
        SET materia = $2, titulo = $3, fecha = $4, porcentaje = $5,
            descripcion = $6, clase = $7, updated_at = now()
        WHERE id = $1
        RETURNING *
        "#,
    )
        .persistent(false)
        .bind(actividad_id)
        .bind(materia)
        .bind(titulo)
        .bind(fecha)
        .bind(porcentaje)
        .bind(descripcion)
        .bind(&clase)
        .fetch_one(pool)
        .await?;

    Ok(Some(row))
}

pub async fn delete_actividad(pool: &PgPool, actividad_id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM actividades WHERE id = $1")
        .persistent(false)
        .bind(actividad_id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn get_nota(pool: &PgPool, actividad_id: Uuid) -> Result<Option<NotaRow>, sqlx::Error> {
    sqlx::query_as::<_, NotaRow>("SELECT * FROM notas WHERE actividad_id = $1")
        .persistent(false)
        .bind(actividad_id)
        .fetch_optional(pool)
        .await
}

pub async fn upsert_nota(
    pool: &PgPool,
    actividad_id: Uuid,
    contenido: &str,
) -> Result<NotaRow, sqlx::Error> {
    sqlx::query_as::<_, NotaRow>(
        r#"
        INSERT INTO notas (actividad_id, contenido)
        VALUES ($1, $2)
        ON CONFLICT (actividad_id) DO UPDATE
        SET contenido = $2, updated_at = now()
        RETURNING *
        "#,
    )
        .persistent(false)
        .bind(actividad_id)
        .bind(contenido)
        .fetch_one(pool)
        .await
}

pub async fn delete_nota(pool: &PgPool, actividad_id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM notas WHERE actividad_id = $1")
        .persistent(false)
        .bind(actividad_id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

fn get_clase_materia(materia: &str) -> String {
    match materia {
        "Matemática I" => "matematica".to_string(),
        "Teología" => "teologia".to_string(),
        "Filosofía" => "filosofia".to_string(),
        "Técnicas de Redacción" => "redaccion".to_string(),
        _ => String::new(), // Desarrollo Personal has no special class
    }
}