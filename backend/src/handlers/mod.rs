use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::db;
use crate::models::*;

pub async fn get_calendario(State(pool): State<PgPool>) -> impl IntoResponse {
    match db::get_full_calendario(&pool).await {
        Ok(calendario) => (
            StatusCode::OK,
            Json(serde_json::to_value(calendario).unwrap()),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("Error fetching calendario: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Error al cargar el calendario"
                })),
            )
                .into_response()
        }
    }
}

pub async fn create_actividad(
    State(pool): State<PgPool>,
    Path(semana_id): Path<Uuid>,
    Json(input): Json<CreateActividad>,
) -> impl IntoResponse {
    if input.materia.trim().is_empty() || input.titulo.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Materia y título son requeridos"
            })),
        )
            .into_response();
    }

    match db::semana_exists(&pool, semana_id).await {
        Ok(false) => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": "Semana no encontrada"
                })),
            )
                .into_response();
        }
        Ok(true) => {}
        Err(e) => {
            tracing::error!("Error checking semana existence: {e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Error al validar la semana"
                })),
            )
                .into_response();
        }
    }

    match db::create_actividad(&pool, semana_id, &input).await {
        Ok(actividad) => (
            StatusCode::CREATED,
            Json(serde_json::json!({
                "id": actividad.id,
                "materia": actividad.materia,
                "titulo": actividad.titulo,
                "fecha": actividad.fecha,
                "porcentaje": actividad.porcentaje,
                "descripcion": actividad.descripcion,
                "clase": actividad.clase,
            })),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("Error creating actividad: {e}");

            if e.as_database_error()
                .and_then(|db_err| db_err.code())
                .as_deref()
                == Some("23503")
            {
                return (
                    StatusCode::NOT_FOUND,
                    Json(serde_json::json!({
                        "error": "Semana no encontrada"
                    })),
                )
                    .into_response();
            }

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Error al crear actividad"
                })),
            )
                .into_response()
        }
    }
}

pub async fn update_actividad(
    State(pool): State<PgPool>,
    Path(actividad_id): Path<Uuid>,
    Json(input): Json<UpdateActividad>,
) -> impl IntoResponse {
    match db::update_actividad(&pool, actividad_id, &input).await {
        Ok(Some(actividad)) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "id": actividad.id,
                "materia": actividad.materia,
                "titulo": actividad.titulo,
                "fecha": actividad.fecha,
                "porcentaje": actividad.porcentaje,
                "descripcion": actividad.descripcion,
                "clase": actividad.clase,
            })),
        )
            .into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "Actividad no encontrada"
            })),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("Error updating actividad: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Error al actualizar actividad"
                })),
            )
                .into_response()
        }
    }
}

pub async fn delete_actividad(
    State(pool): State<PgPool>,
    Path(actividad_id): Path<Uuid>,
) -> impl IntoResponse {
    match db::delete_actividad(&pool, actividad_id).await {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "Actividad no encontrada"
            })),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("Error deleting actividad: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Error al eliminar actividad"
                })),
            )
                .into_response()
        }
    }
}

pub async fn get_nota(
    State(pool): State<PgPool>,
    Path(actividad_id): Path<Uuid>,
) -> impl IntoResponse {
    match db::get_nota(&pool, actividad_id).await {
        Ok(Some(nota)) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "id": nota.id,
                "actividad_id": nota.actividad_id,
                "contenido": nota.contenido,
            })),
        )
            .into_response(),
        Ok(None) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "actividad_id": actividad_id,
                "contenido": "",
            })),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("Error fetching nota: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Error al cargar nota"
                })),
            )
                .into_response()
        }
    }
}

pub async fn upsert_nota(
    State(pool): State<PgPool>,
    Path(actividad_id): Path<Uuid>,
    Json(input): Json<UpsertNota>,
) -> impl IntoResponse {
    if input.contenido.trim().is_empty() {
        match db::delete_nota(&pool, actividad_id).await {
            Ok(_) => {
                return (
                    StatusCode::OK,
                    Json(serde_json::json!({
                        "actividad_id": actividad_id,
                        "contenido": "",
                    })),
                )
                    .into_response()
            }
            Err(e) => {
                tracing::error!("Error deleting nota: {e}");
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "error": "Error al eliminar nota"
                    })),
                )
                    .into_response();
            }
        }
    }

    match db::upsert_nota(&pool, actividad_id, &input.contenido).await {
        Ok(nota) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "id": nota.id,
                "actividad_id": nota.actividad_id,
                "contenido": nota.contenido,
            })),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("Error upserting nota: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Error al guardar nota"
                })),
            )
                .into_response()
        }
    }
}