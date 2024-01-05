use actix_web::{post, get, web::Data, web::Json, web::ServiceConfig, HttpResponse, HttpServer, App};
use bcrypt::{hash, DEFAULT_COST};
use actix_cors::Cors;
use leptos::*;
use libsql_client::{Client, Config};
use serde::Deserialize;
use shuttle_actix_web::ShuttleActixWeb;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize)]
struct UserRegistration {
    id: u32,
    username: String,
    password: String,
}

#[post("/register")]
async fn register_user(
    client: Data<Arc<Mutex<Client>>>,
    form: Json<UserRegistration>,
) -> HttpResponse {
    let registration_data: UserRegistration = form.into_inner();
    let hash_pwd: String = match hash(registration_data.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return HttpResponse::InternalServerError().body("Error hashing password"),
    };

    let sql_cmd: String = format!(
        "INSERT INTO users (id, username, password) VALUES ({}, '{}', '{}')",
        registration_data.id, registration_data.username, hash_pwd
    );

    let client: tokio::sync::MutexGuard<'_, Client> = client.lock().await;
    match client.execute(sql_cmd.as_str()).await {
        Ok(_) => HttpResponse::Ok().body("User registered"),
        Err(_) => HttpResponse::InternalServerError().body("Error registering user"),
    }
}

#[get("/register")]
async fn register_page() -> HttpResponse {
    let html_content: &str = r##"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <title>Register User</title>
            <script src="https://unpkg.com/htmx.org"></script>
        </head>
        <body>
            <h1>User Registration</h1>
            <form hx-post="http://localhost:8000/register" hx-encoding="application/json" hx-target="#response" hx-trigger="submit">
                <input type="text" name="username" placeholder="Username" required>
                <input type="password" name="password" placeholder="Password" required>
                <button type="submit">Register</button>
            </form>
            <div id="response"></div>
        </body>
        </html>
    "##;

    HttpResponse::Ok().content_type("text/html").body(html_content)
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let client = Client::from_config(Config {
        url: url::Url::parse("libsql://login-system-guibarbosacrvg.turso.io").unwrap(),
        auth_token: None,
    })
    .await
    .unwrap();

    let shared_client = Arc::new(Mutex::new(client));

    let config = move |cfg: &mut ServiceConfig| {
        let cors = Cors::permissive();
        cfg.app_data(Data::new(shared_client.clone()))
           .service(register_user)
           .service(register_page);
    };

    ShuttleActixWeb::new(config) // Wrap the configuration in ActixWebService
}
