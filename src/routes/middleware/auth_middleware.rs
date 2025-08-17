use actix_web::{
    Error,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    http::header::AUTHORIZATION,
    middleware::Next,
};

use crate::utils::{api_response, jwt::decode_jwt};

pub async fn check_auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let auth = req.headers().get(AUTHORIZATION);

    if auth.is_none() {
        return Err(Error::from(api_response::ApiResponse::new(
            401,
            "Unathorized".to_string(),
        )));
    };

    let token = auth.unwrap().to_str().unwrap().to_owned();
    let claim = decode_jwt(token).unwrap();

    next.call(req)
        .await
        .map_err(|err: Error| Error::from(api_response::ApiResponse::new(500, err.to_string())))
}
