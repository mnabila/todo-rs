use axum::{
    extract::{Extension, Request},
    middleware::Next,
    response::Response,
};

use crate::{
    infrastructure::security::jwt::JwtClaims, presentation::restapi::response::ApiResponse,
};

pub async fn jwt_middleware(
    Extension(secret): Extension<String>,
    mut request: Request,
    next: Next,
) -> Result<Response, ApiResponse<()>> {
    let token = request
        .headers()
        .get("Authorization")
        .and_then(|val| val.to_str().ok())
        .and_then(|data| data.strip_prefix("Bearer "))
        .ok_or(ApiResponse::unauthorized("Authorization not found"))?;

    let claims = JwtClaims::decode(token.to_string(), &secret)
        .map_err(|_| ApiResponse::unauthorized("Authorization not found"))?;

    request.extensions_mut().insert(claims);
    Ok(next.run(request).await)
}
