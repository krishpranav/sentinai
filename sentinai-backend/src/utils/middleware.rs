use axum::{extract::Request, http::header, middleware::Next, response::Response};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct RequestId(pub String);

pub async fn request_id_middleware(mut req: Request, next: Next) -> Response {
    let request_id = Uuid::new_v4().to_string();

    req.extensions_mut().insert(RequestId(request_id.clone()));

    let mut response = next.run(req).await;

    response.headers_mut().insert(
        header::HeaderName::from_static("x-request-id"),
        request_id.parse().unwrap(),
    );

    response
}
