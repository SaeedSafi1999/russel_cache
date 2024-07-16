use actix_web::http::uri::Port;
use actix_web::{web, App, HttpServer, HttpResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::cache::Cache; // Adjust based on your module structure

// Data structure for API responses
#[derive(Serialize)]
struct ApiResponse<T> {
    status: String,
    data: T,
}

impl<T> ApiResponse<T> {
    fn new(data: T) -> Self {
        ApiResponse {
            status: "success".into(),
            data,
        }
    }
}

// Handler function for `set` API
#[derive(Deserialize)]
struct SetRequest {
    cluster: String,
    key: String,
    value: Vec<u8>,
}

pub async fn set(cache: web::Data<Arc<Mutex<Cache>>>, payload: web::Json<SetRequest>) -> HttpResponse {
    let SetRequest { cluster, key, value } = &*payload;

    cache.lock().unwrap().set(cluster.clone(), key.clone(), value.clone());
    HttpResponse::Ok().json(ApiResponse::new("Set operation successful"))
}

// Handler function for `get` API
pub async fn get(cache: web::Data<Arc<Mutex<Cache>>>, info: web::Path<(String, String)>) -> HttpResponse {
    let (cluster, key) = info.into_inner();
    match cache.lock().unwrap().get(&cluster, &key) {
        Some(value) => HttpResponse::Ok().json(ApiResponse::new(value)),
        None => HttpResponse::NotFound().json(ApiResponse::new("Key not found")),
    }
}

// Handler function for `delete` API
pub async fn delete(cache: web::Data<Arc<Mutex<Cache>>>, info: web::Path<(String, String)>) -> HttpResponse {
    let (cluster, key) = info.into_inner();
    cache.lock().unwrap().delete(&cluster, &key);
    HttpResponse::Ok().json(ApiResponse::new("Delete operation successful"))
}

// Handler function for `clear_cluster` API
pub async fn clear_cluster(cache: web::Data<Arc<Mutex<Cache>>>, cluster: web::Path<String>) -> HttpResponse {
    cache.lock().unwrap().clear_cluster(&cluster);
    HttpResponse::Ok().json(ApiResponse::new("Clear cluster operation successful"))
}

// Handler function for `clear_all` API
pub async fn clear_all(cache: web::Data<Arc<Mutex<Cache>>>) -> HttpResponse {
    cache.lock().unwrap().clear_all();
    HttpResponse::Ok().json(ApiResponse::new("Clear all operation successful"))
}

// Handler function for `get_all_clusters` API
pub async fn get_all_clusters(cache: web::Data<Arc<Mutex<Cache>>>) -> HttpResponse {
    let clusters = cache.lock().unwrap().get_all_clusters();
    HttpResponse::Ok().json(ApiResponse::new(clusters))
}

pub async fn get_port(cache: web::Data<Arc<Mutex<Cache>>>) -> HttpResponse {
    let port = cache.lock().unwrap().get_default_port();
    HttpResponse::Ok().json(ApiResponse::new(port))
}

// Configure and start the HTTP server
pub async fn run_server(cache: Arc<Mutex<Cache>>) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(cache.clone())) // Share cache across handlers
            .route("/api/set", web::post().to(set))
            .route("/api/get/{cluster}/{key}", web::get().to(get))
            .route("/api/delete/{cluster}/{key}", web::delete().to(delete))
            .route("/api/clear_cluster/{cluster}", web::delete().to(clear_cluster))
            .route("/api/clear_all", web::delete().to(clear_all))
            .route("/api/get_clusters", web::get().to(get_all_clusters))
            .route("/api/port", web::get().to(get_port))
    })
    .bind("127.0.0.1:5022")? // Bind to localhost on port 5022
    .run()
    .await
}
