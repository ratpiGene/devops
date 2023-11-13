use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use dotenv::dotenv;
use serde::Serialize;
use std::env;

#[derive(Serialize)] // Ajout de derive(Serialize) pour rendre les structures JSON-sérialisables
struct Headers {
    method: String,
    uri: String,
    host: Option<String>, // Option pour prendre en charge les cas où le champ n'est pas défini
    user_agent: Option<String>, // Option pour prendre en charge les cas où le champ n'est pas défini
}

async fn ping_handler(req: HttpRequest) -> HttpResponse {
    let full_url = req.uri().to_string(); // Récupérer l'URL complète
    println!("Received GET request on URL: {}", full_url);

    // Récupération des headers de la requête
    let headers = Headers {
        method: format!("{:?}", req.method()),
        uri: full_url.clone(), // Utiliser l'URL complète
        host: req.headers().get("host").map(|value| value.to_str().unwrap_or_default().to_string()),
        user_agent: req.headers().get("user-agent").map(|value| value.to_str().unwrap_or_default().to_string()),
    };

    // Réponse au format JSON avec les headers
    HttpResponse::Ok().json(headers)
}

async fn default_handler(req: HttpRequest) -> HttpResponse {
    let full_url = req.uri().to_string(); // Récupérer l'URL complète
    println!("Received request on an unsupported route. URL: {}", full_url);

    HttpResponse::NotFound().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port: u16 = env::var("PING_LISTEN_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080);

    println!("Server is listening on port: {}", port);

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/ping").route(web::get().to(ping_handler)))
            .default_service(web::route().to(default_handler))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
