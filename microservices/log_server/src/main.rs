#[macro_use]
extern crate log;
extern crate lazy_static;

use lazy_static::lazy_static;
use prometheus::{IntCounter, TextEncoder, Encoder, Registry};
use actix_web::{web, App, HttpServer, HttpResponse, Error};
use serde_derive::{Serialize, Deserialize};
use actix_web::web::Bytes;
use futures::future::{ready, Ready};
use serde_json::Error as SerdeJsonError;
use env_logger::Env;
use actix_logger::Logger;
use log::{info, error};
use chrono::Utc;
use std::str::Utf8Error;
use std::string::FromUtf8Error;
use actix_web::http::StatusCode;
use std::fmt::Display;
use actix_web::rt::time::Instant;
use prometheus::{IntCounterVec, HistogramVec, register_int_counter_vec, register_histogram_vec};

lazy_static! {
    pub static ref LOG_ENTRIES_TOTAL: LogEntriesTotal = LogEntriesTotal::new();
}

lazy_static! {
    pub static ref ENDPOINT_REQUESTS: IntCounterVec = register_int_counter_vec!(
        "http_endpoint_requests",
        "Number of requests per endpoint",
        &["method", "endpoint"]
    )
    .expect("Failed to create counter");
}

lazy_static! {
    // Métrique pour enregistrer le nombre total de requêtes
    pub static ref REQUESTS_TOTAL: IntCounterVec = register_int_counter_vec!(
        "http_requests_total",
        "Total number of HTTP requests",
        &["method"]
    )
    .expect("Failed to create counter");

    // Métrique pour enregistrer la durée des requêtes
    pub static ref REQUESTS_DURATION: HistogramVec = register_histogram_vec!(
        "http_requests_duration_seconds",
        "HTTP request duration in seconds",
        &["method"]
    )
    .expect("Failed to create histogram");

    // Métrique pour enregistrer l'ID, le contenu, l'IP, le système d'exploitation, le navigateur, la localisation et l'horodatage
    pub static ref LOG_ENTRIES: HistogramVec = register_histogram_vec!(
        "log_entry_info",
        "Log entry information",
        &["id", "content", "ip", "os", "browser", "location", "timestamp"]
    )
    .expect("Failed to create histogram");
}

#[derive(Debug)]
struct Utf8ErrorWrapper(Utf8Error);

impl Display for Utf8ErrorWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct FromUtf8ErrorWrapper(FromUtf8Error);

impl Display for FromUtf8ErrorWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl actix_web::ResponseError for FromUtf8ErrorWrapper {}

#[derive(Debug)]
struct SerdeJsonErrorWrapper(SerdeJsonError);

impl Display for SerdeJsonErrorWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
enum CustomError {
    Utf8ErrorWrapper(Utf8ErrorWrapper),
    FromUtf8ErrorWrapper(FromUtf8ErrorWrapper),
    SerdeJsonErrorWrapper(SerdeJsonErrorWrapper),
}

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomError::Utf8ErrorWrapper(inner) => write!(f, "Utf8ErrorWrapper: {}", inner),
            CustomError::FromUtf8ErrorWrapper(inner) => write!(f, "FromUtf8ErrorWrapper: {}", inner),
            CustomError::SerdeJsonErrorWrapper(inner) => write!(f, "SerdeJsonErrorWrapper: {}", inner),
        }
    }
}

#[derive(Debug)]
struct PrometheusErrorWrapper(prometheus::Error);

impl actix_web::ResponseError for PrometheusErrorWrapper {}

impl Display for PrometheusErrorWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Prometheus error: {:?}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Location {
    country: Option<String>,
    city: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct LogEntry {
    id: u32,
    content: String,
    ip: Option<String>,
    os: Option<String>,
    browser: Option<String>,
    location: Option<Location>,
    timestamp: String,
}

#[derive(Debug)]
struct BytesWrapper(Bytes);

impl actix_web::FromRequest for BytesWrapper {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        ready(Ok(BytesWrapper(Bytes::new())))
    }
}

#[derive(Debug, Clone)]
pub struct LogEntriesTotal {
    pub total_log_entries: IntCounter,
}

impl LogEntriesTotal {
    pub fn new() -> Self {
        LogEntriesTotal {
            total_log_entries: IntCounter::new("total_log_entries", "Total number of log entries")
                .expect("Failed to create IntCounter"),
        }
    }
}

async fn handle_metrics(registry: web::Data<Registry>) -> Result<HttpResponse, Error> {
    info!("Handling metrics request");

    let encoder = TextEncoder::new();
    let metric_families = registry.gather();

    let mut buffer = vec![];
    if let Err(e) = encoder.encode(&metric_families, &mut buffer) {
        error!("Error encoding metrics: {:?}", e);
        return Ok(HttpResponse::InternalServerError().finish());
    }

    let prometheus_metrics: String = match String::from_utf8(buffer) {
        Ok(s) => s,
        Err(e) => {
            error!("Error converting bytes to String: {:?}", e);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    info!("Metrics response generated successfully");

    Ok(HttpResponse::Ok().body(prometheus_metrics))
}

async fn handle_post(req: actix_web::HttpRequest, payload: Bytes) -> Result<HttpResponse, Error> {
    let allowed_ip: String = "127.0.0.1".to_string();

    if req.connection_info().peer_addr().unwrap_or_default().to_string() != allowed_ip {
        return Ok(HttpResponse::Forbidden().finish());
    }

    let ip = req.connection_info().peer_addr().unwrap_or_default().to_string();
    let start_time = Instant::now();

    let content = match String::from_utf8(payload.to_vec()) {
        Ok(c) => c,
        Err(_) => return Ok(HttpResponse::BadRequest().finish()),
    };

    let mut log_entry: Result<LogEntry, _> = serde_json::from_str(&content);

    if let Ok(ref mut entry) = log_entry {
        entry.ip = Some(ip.clone());
        entry.timestamp = Utc::now().to_rfc3339();
    }

    info!(
        "Received request: \n{:?}\n  payload: {}",
        req,
        String::from_utf8_lossy(&payload)
    );

    let content = match String::from_utf8(payload.to_vec()) {
        Ok(c) => c,
        Err(_) => return Ok(HttpResponse::BadRequest().finish()),
    };

    let mut log_entry: Result<LogEntry, _> = serde_json::from_str(&content);

    if let Ok(ref mut entry) = log_entry {
        entry.ip = Some(ip);
        entry.timestamp = Utc::now().to_rfc3339();
    }

    match log_entry {
        Ok(entry) => {
            info!("Received log entry: {:?}", &entry);

            REQUESTS_TOTAL
                .with_label_values(&[&req.method().as_str()])
                .inc();

            LOG_ENTRIES_TOTAL.total_log_entries.inc();

            REQUESTS_DURATION
                .with_label_values(&[&req.method().as_str()])
                .observe(start_time.elapsed().as_secs_f64());

            LOG_ENTRIES
                .with_label_values(&[
                    &entry.id.to_string(),
                    &entry.content,
                    &entry.ip.unwrap_or_default(),
                    &entry.os.unwrap_or_default(),
                    &entry.browser.unwrap_or_default(),
                    &format!("{:?}", entry.location.unwrap_or_default()),
                    &entry.timestamp,
                ])
                .observe(start_time.elapsed().as_secs_f64());

            let encoder = TextEncoder::new();
            let metric_families = prometheus::gather();

            let mut buffer = vec![];
            encoder.encode(&metric_families, &mut buffer).unwrap();

            let prometheus_metrics = String::from_utf8(buffer).unwrap();
            Ok(HttpResponse::Ok().body(prometheus_metrics))
        }
        Err(e) => {
            error!("Failed to deserialize JSON: {:?}", e);
            Ok(HttpResponse::BadRequest().finish())
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Ajoutez les métriques à la configuration de l'application
    let registry = Registry::new();
    registry.register(Box::new(REQUESTS_TOTAL.clone())).unwrap();
    registry.register(Box::new(ENDPOINT_REQUESTS.clone())).unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(web::resource("/log").route(web::post().to(handle_post)))
            .route("/metrics", web::get().to(handle_metrics))
            .data(registry.clone()) // Passez le registre à la fonction handle_metrics

            // Enregistrez vos métriques ici
            .app_data(web::Data::new(REQUESTS_TOTAL.clone()))
            .app_data(web::Data::new(ENDPOINT_REQUESTS.clone()))
            .app_data(web::Data::new(registry.clone())) 
    })
    .bind("127.0.0.1:9090")?
    .run()
    .await
}
