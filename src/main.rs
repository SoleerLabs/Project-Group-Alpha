pub use self::errors::{Error, Result};
use axum::{
    middleware,
    routing::{get, get_service},
    Router,
};
use std::net::SocketAddr;
use tower_http::services::ServeDir;
mod ctx;
mod errors;
mod web;
use web::db::{new_db_pool, Db};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let db: Db = new_db_pool().await.expect("Failed to create database pool");

    let public_routes = web::routes_auth::routes(db.clone());

    let protected_routes = Router::new()
        .route("/me", get(web::routes_auth::me))
        .merge(web::project::routes::routes(db.clone()))
        .merge(web::task::routes::routes(db.clone()))
        .merge(web::user::routes::routes(db.clone()))
        .route_layer(middleware::from_fn_with_state(
            db.clone(),
            web::mw_auth::mw_auth,
        ));

    let api_routes = Router::new()
        .merge(public_routes)
        .merge(protected_routes);

    let routes_all = Router::new()
        .nest("/api", api_routes)
        .fallback_service(get_service(ServeDir::new("./")));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("-->>> Listening on http://{}", addr);
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        routes_all,
    )
    .await
    .unwrap();
}