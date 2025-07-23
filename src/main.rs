use axum::{
    routing::{get, get_service},
    Router,
    response::{Html, IntoResponse},
    extract::{Path, Query},
};
use std::net::SocketAddr;
use tower_http::services::ServeDir;
pub use self::errors::{Error, Result};
mod errors;
mod web;
#[tokio::main]
async fn main() {
    let routes_all = Router::new().merge(routes_hello().merge(web::routes_login::route()).fallback_service(routes_static()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("-->>> Listening on http://{}", addr);
//region ----start server----
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        routes_all,
    )
    .await
    .unwrap();
}

//create ----params----
#[derive(Debug, serde::Deserialize)]
struct HelloParams {
    pub name:Option<String>
}
// ----handler----
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello {params:?}", "HANDLER");
    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("<h1>Hello, {name}</h1>"))
}

// -----path params ----
// e.g heelo2/Mike
async fn handler_hello2(Path(name):Path<String>) -> impl IntoResponse{
    println!("->{:<12} - handler_hello2 {name:?}", "HANDLER");
    Html(format!("<h1>Hello, {name}</h1>"))
}

//Region -----Static Routing----
fn routes_static() -> Router {
   Router::new().nest_service("/nest", get_service(ServeDir::new("./")))
}

//Region ----Routes Hello----
fn routes_hello()-> Router { Router::new().route("/hello", get(handler_hello)).route("/hello2/{name}", get(handler_hello2)).route("/users", get(web::user::get_users)) }