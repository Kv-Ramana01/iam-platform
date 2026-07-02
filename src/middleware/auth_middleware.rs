use crate::{auth::jwt, state::AppState};
use axum::{
    extract::{Request, State},
    http::{StatusCode, header::AUTHORIZATION},
    middleware::Next,
    response::Response,
};

use crate::repositories::session_repository;

use uuid::Uuid;

pub async fn auth(
    State(state): State<AppState>,
    mut request: Request,
    next: Next, //the handler
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let claims = jwt::verify_token(token).map_err(|_| StatusCode::UNAUTHORIZED)?;

    let session_id = Uuid::parse_str(&claims.sid).map_err(|_| StatusCode::UNAUTHORIZED)?;

    let session = session_repository::find_session_by_id(&state.pool, session_id)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let session = match session {
        Some(session) => session,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    if !session.is_active {
        return Err(StatusCode::UNAUTHORIZED);
    }

    use chrono::Utc;

    if session.expires_at < Utc::now() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}
