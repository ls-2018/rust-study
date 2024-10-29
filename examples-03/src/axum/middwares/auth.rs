use super::TokenVerify;
use axum::{
    extract::{FromRequestParts, Query, Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use serde::Deserialize;
use tracing::warn;

#[derive(Debug, Deserialize)]
struct Params {
    token: String,
}

pub async fn verify_token<T>(State(state): State<T>, req: Request, next: Next) -> Response
where
    T: TokenVerify + Clone + Send + Sync + 'static,
{
    let (mut parts, body) = req.into_parts();
    let token =
        match TypedHeader::<Authorization<Bearer>>::from_request_parts(&mut parts, &state).await {
            Ok(TypedHeader(Authorization(bearer))) => bearer.token().to_string(),
            Err(e) => {
                if e.is_missing() {
                    match Query::<Params>::from_request_parts(&mut parts, &state).await {
                        Ok(params) => params.token.clone(),
                        Err(e) => {
                            let msg = format!("parse query params failed: {}", e);
                            warn!(msg);
                            return (StatusCode::UNAUTHORIZED, msg).into_response();
                        }
                    }
                } else {
                    let msg = format!("parse Authorization header failed: {}", e);
                    warn!(msg);
                    return (StatusCode::UNAUTHORIZED, msg).into_response();
                }
            }
        };

    let req = match state.verify(&token) {
        Ok(user) => {
            let mut req = Request::from_parts(parts, body);
            req.extensions_mut().insert(user);
            req
        }
        Err(e) => {
            let msg = format!("verify token failed: {:?}", e);
            warn!(msg);
            return (StatusCode::FORBIDDEN, msg).into_response();
        }
    };

    next.run(req).await
}
const JWT_DURATION: u64 = 60 * 60 * 24 * 7;
const JWT_ISS: &str = "chat_server";
const JWT_AUD: &str = "chat_web";
#[cfg(test)]
mod tests {
    use super::{verify_token, TokenVerify, JWT_AUD, JWT_DURATION, JWT_ISS};
    use crate::axum::User;
    use crate::{KEY_PEM, PUB_CEM};
    use anyhow::Result;
    use axum::extract::Request;
    use axum::response::IntoResponse;
    use axum::{body::Body, middleware::from_fn_with_state, routing::get, Router};
    use http::StatusCode;
    use jwt_simple::prelude::*;
    use jwt_simple::prelude::{
        Claims, Duration, Ed25519KeyPair, Ed25519PublicKey, EdDSAKeyPairLike, VerificationOptions,
    };
    use std::sync::Arc;
    use tower::ServiceExt;

    #[derive(Clone)]
    struct AppState(Arc<AppStateInner>);

    struct AppStateInner {
        ek: Ed25519KeyPair,
        dk: Ed25519PublicKey,
    }

    impl TokenVerify for AppState {
        type Error = ();

        fn verify(&self, token: &str) -> Result<User, Self::Error> {
            let opts = VerificationOptions {
                allowed_issuers: Some(HashSet::from_strings(&[JWT_ISS])),
                allowed_audiences: Some(HashSet::from_strings(&[JWT_AUD])),
                ..Default::default()
            };
            let claims = self.0.dk.verify_token::<User>(token, Some(opts));
            if claims.is_err() {
                Err(())
            } else {
                Ok(claims.ok().unwrap().custom)
            }
        }
    }

    async fn handler(_req: Request) -> impl IntoResponse {
        (StatusCode::OK, "ok")
    }

    #[tokio::test]
    async fn verify_token_middleware_should_work() -> Result<()> {
        let ek = Ed25519KeyPair::from_pem(KEY_PEM)?;
        let dk = Ed25519PublicKey::from_pem(PUB_CEM)?;
        let state = AppState(Arc::new(AppStateInner { ek, dk }));

        let user = User::default();
        let claims = Claims::with_custom_claims(user, Duration::from_secs(JWT_DURATION));
        let claims = claims.with_issuer(JWT_ISS).with_audience(JWT_AUD);
        let token = state.0.ek.sign(claims)?;

        let app = Router::new()
            .route("/", get(handler))
            .layer(from_fn_with_state(state.clone(), verify_token::<AppState>))
            .with_state(state);

        // good token
        let req = Request::builder()
            .uri("/")
            .header("Authorization", format!("Bearer {}", token))
            .body(Body::empty())?;
        let res = app.clone().oneshot(req).await?;
        assert_eq!(res.status(), StatusCode::OK);

        // good token in query params
        let req = Request::builder()
            .uri(format!("/?token={}", token))
            .body(Body::empty())?;
        let res = app.clone().oneshot(req).await?;
        assert_eq!(res.status(), StatusCode::OK);

        // no token
        let req = Request::builder().uri("/").body(Body::empty())?;
        let res = app.clone().oneshot(req).await?;
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

        // bad token
        let req = Request::builder()
            .uri("/")
            .header("Authorization", "Bearer bad-token")
            .body(Body::empty())?;
        let res = app.clone().oneshot(req).await?;
        assert_eq!(res.status(), StatusCode::FORBIDDEN);

        // bad token in query params
        let req = Request::builder()
            .uri("/?token=bad-token")
            .body(Body::empty())?;
        let res = app.oneshot(req).await?;
        assert_eq!(res.status(), StatusCode::FORBIDDEN);

        Ok(())
    }
}
