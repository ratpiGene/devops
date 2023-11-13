use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use std::env;

#[derive(Debug)]
struct Headers {
    method: String,
    uri: String,
    host: Option<String>,
    user_agent: Option<String>,
}

impl Headers {
    fn new(req: &HttpRequest) -> Self {
        let full_url = format!("{}", req.uri());
        let method = format!("{:?}", req.method());
        let host = req.headers().get("host").map(|value| value.to_str().unwrap_or_default().to_string());
        let user_agent = req.headers().get("user-agent").map(|value| value.to_str().unwrap_or_default().to_string());

        Headers { method, uri: full_url, host, user_agent }
    }

    fn to_json_string(&self) -> String {
        format!(
            r#"{{"method":"{}","uri":"{}","host":{:?},"user_agent":{:?}}}"#,
            self.method, self.uri, self.host, self.user_agent
        )
    }
}

async fn ping_handler(req: HttpRequest) -> HttpResponse {
    let headers = Headers::new(&req);

    // Réponse au format JSON avec les headers
    HttpResponse::Ok().body(headers.to_json_string())
}

async fn default_handler(req: HttpRequest) -> HttpResponse {
    let full_url = format!("{}", req.uri());
    println!("Received request on an unsupported route. URL: {}", full_url);

    HttpResponse::NotFound().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Récupérer le port à partir de la variable d'environnement
    let port: u16 = env::var("PING_LISTEN_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080);

    println!("Server is listening on port: {}", port);

    // Créer le serveur Actix-web
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/ping").route(web::get().to(ping_handler)))
            .default_service(web::route().to(default_handler))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
