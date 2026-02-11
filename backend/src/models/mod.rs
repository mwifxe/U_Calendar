use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct PeriodoRow {
    pub id: Uuid,
    pub numero: String,
    pub nombre: String,
    pub info: String,
    pub orden: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct SemanaRow {
    pub id: Uuid,
    pub periodo_id: Uuid,
    pub codigo: String,
    pub titulo: String,
    pub fecha: String,
    pub es_parcial: bool,
    pub es_vacaciones: bool,
    pub nota_especial: Option<String>,
    pub orden: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct ActividadRow {
    pub id: Uuid,
    pub semana_id: Uuid,
    pub materia: String,
    pub titulo: String,
    pub fecha: Option<String>,
    pub porcentaje: Option<String>,
    pub descripcion: Option<String>,
    pub clase: String,
    pub orden: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct ParcialRow {
    pub id: Uuid,
    pub semana_id: Uuid,
    pub materia: String,
    pub orden: i16,
}

#[derive(Debug, FromRow)]
pub struct NotaRow {
    pub id: Uuid,
    pub actividad_id: Uuid,
    pub contenido: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct CalendarioResponse {
    pub periodos: Vec<PeriodoResponse>,
}

#[derive(Debug, Serialize)]
pub struct PeriodoResponse {
    pub id: Uuid,
    pub numero: String,
    pub nombre: String,
    pub info: String,
    pub semanas: Vec<SemanaResponse>,
}

#[derive(Debug, Serialize)]
pub struct SemanaResponse {
    pub id: Uuid,
    pub codigo: String,
    pub titulo: String,
    pub fecha: String,
    pub es_parcial: bool,
    pub es_vacaciones: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nota_especial: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parciales: Option<Vec<String>>,
    pub actividades: Vec<ActividadResponse>,
}

#[derive(Debug, Serialize)]
pub struct ActividadResponse {
    pub id: Uuid,
    pub materia: String,
    pub titulo: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fecha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub porcentaje: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descripcion: Option<String>,
    pub clase: String,
    pub tiene_notas: bool,
}

#[derive(Debug, Serialize)]
pub struct NotaResponse {
    pub id: Uuid,
    pub actividad_id: Uuid,
    pub contenido: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateActividad {
    pub materia: String,
    pub titulo: String,
    pub fecha: Option<String>,
    pub porcentaje: Option<String>,
    pub descripcion: Option<String>,
    pub clase: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateActividad {
    pub materia: Option<String>,
    pub titulo: Option<String>,
    pub fecha: Option<String>,
    pub porcentaje: Option<String>,
    pub descripcion: Option<String>,
    pub clase: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpsertNota {
    pub contenido: String,
}