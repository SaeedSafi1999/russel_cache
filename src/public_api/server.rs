use actix_web::{web, App, HttpServer, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use crate::cache::Cache;


#[derive(Serialize)]
struct ApiResponse<T> {
    is_success: bool,
    data: T,
}


impl<T> ApiResponse<T> {
    fn ok(data: T) -> Self {
        ApiResponse {
            is_success: true,
            data,
        }
    }
    fn fail(data: T) -> Self {
        ApiResponse {
            is_success: false,
            data,
        }
    }
}


#[derive(Deserialize)]
struct SetRequest {
    cluster: String,
    key: String,
    value: String,
}


pub async fn set(cache: web::Data<Arc<Mutex<Cache>>>, payload: web::Json<SetRequest>) -> HttpResponse {
    let SetRequest { cluster, key, value } = &*payload;
    let set_value = value.as_bytes();
    cache.lock().unwrap().set(cluster.clone(), key.clone(), Vec::from(set_value));
    HttpResponse::Ok().json(ApiResponse::ok("Set operation successful"))
}


pub async fn get(cache: web::Data<Arc<Mutex<Cache>>>, info: web::Path<(String, String)>) -> HttpResponse {
    let (cluster, key) = info.into_inner();
    match cache.lock().unwrap().get(&cluster, &key) {
        Some(value) => HttpResponse::Ok().json(ApiResponse::ok(value)),
        None => HttpResponse::NotFound().json(ApiResponse::fail("Key not found")),
    }
}

pub async fn delete(cache: web::Data<Arc<Mutex<Cache>>>, info: web::Path<(String, String)>) -> HttpResponse {
    let (cluster, key) = info.into_inner();
    cache.lock().unwrap().delete(&cluster, &key);
    HttpResponse::Ok().json(ApiResponse::ok("Delete operation successful"))
}

pub async fn clear_cluster(cache: web::Data<Arc<Mutex<Cache>>>, cluster: web::Path<String>) -> HttpResponse {
    cache.lock().unwrap().clear_cluster(&cluster);
    HttpResponse::Ok().json(ApiResponse::ok("Clear cluster operation successful"))
}

pub async fn clear_all(cache: web::Data<Arc<Mutex<Cache>>>) -> HttpResponse {
    cache.lock().unwrap().clear_all();
    HttpResponse::Ok().json(ApiResponse::ok("Clear all operation successful"))
}

pub async fn get_all_clusters(cache: web::Data<Arc<Mutex<Cache>>>) -> HttpResponse {
    let clusters = cache.lock().unwrap().get_all_clusters();
    HttpResponse::Ok().json(ApiResponse::ok(clusters))
}

pub async fn get_port(cache: web::Data<Arc<Mutex<Cache>>>) -> HttpResponse {
    let port = cache.lock().unwrap().get_default_port();
    HttpResponse::Ok().json(ApiResponse::ok(port))
}

pub async fn run_server(cache: Arc<Mutex<Cache>>,port_number:String,ip:String) -> std::io::Result<()> {
    let port:String = port_number;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(cache.clone())) 
            .route("/api/set", web::post().to(set))
            .route("/api/get/{cluster}/{key}", web::get().to(get))
            .route("/api/delete/{cluster}/{key}", web::delete().to(delete))
            .route("/api/clear_cluster/{cluster}", web::delete().to(clear_cluster))
            .route("/api/clear_all", web::delete().to(clear_all))
            .route("/api/get_clusters", web::get().to(get_all_clusters))
            .route("/api/port", web::get().to(get_port))
    })
    .bind(format!("{}:{}",ip,port))? // Bind to localhost on port 5022
    .run()
    .await
}
