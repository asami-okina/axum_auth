use axum::{response::Response, routing::get, Router};
use axum_auth::AuthBasic;
use hyper::{header, Body, Server};
use std::net::SocketAddr;

async fn authenticated_handler(AuthBasic((id, password)): AuthBasic) -> Response<Body> {
    let expected_id = "tabiplan_user";
    let expected_password = Some("tabiplan_password".to_string());

    if id != expected_id || password != expected_password {
        return Response::builder()
            .status(401)
            .header(header::WWW_AUTHENTICATE, "Basic realm=\"Secure Area\"")
            .body(Body::empty())
            .unwrap();
    } else {
        return Response::builder()
            .status(200)
            .body(Body::from(format!(
                "User '{}' authenticated successfully",
                id
            )))
            .unwrap();
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/authenticated", get(authenticated_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let server = Server::bind(&addr).serve(app.into_make_service());

    if let Err(error) = server.await {
        eprintln!("Server error: {}", error);
    }
}
