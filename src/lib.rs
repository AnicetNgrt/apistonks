use actix_web::{web::{self, Json}, App, HttpRequest, HttpServer, Responder, HttpResponse};
use actix_web::dev::Server;
use actix_web::http::StatusCode;
use serde::Deserialize;
use std::net::TcpListener;
use std::env;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
            App::new()
                .route("/", web::get().to(greet))
                .route("/hello/{name}", web::get().to(greet))
                .route("/health_check", web::get().to(health_check))
                .route("/mapquest_client_key", web::get().to(mapquest_client_key))
                .route("/gouvfr_geo_fwd", web::get().to(gouvfr_geo_fwd))
        })
        .listen(listener)?
        .run();

    Ok(server)
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

// we can remove req if unused thanks to actix_web
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

async fn mapquest_client_key() -> impl Responder {
    let key = env::var("MAPQUEST_CLIENT_KEY").unwrap_or("not found".to_string());

    

    match &key[..] {
        "not found" => HttpResponse::NotFound().finish(),
        _ => HttpResponse::Ok()
            .content_type("plain/text")
            .body(key)
    }
}

#[derive(Deserialize)]
struct GouvFrGeoFwdBody {
    uri: String
}

async fn gouvfr_geo_fwd(body: Json<GouvFrGeoFwdBody>) -> HttpResponse {
    let response = reqwest::Client::new()
        .get(&format!("https://geo.api.gouv.fr/{}", &body.uri))
        .send()
        .await
        .unwrap();

    let status = response.status().as_u16();
    let body = response.text().await.unwrap();

    match status {
        200 => HttpResponse::Ok()
                .content_type("plain/text")
                .body(body),
        _ => HttpResponse::new(StatusCode::from_u16(status).unwrap_or(StatusCode::NOT_FOUND)),
    }
}