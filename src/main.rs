use actix_web::{web, App, HttpServer};
use sea_orm::{Database, DatabaseConnection};
use slog::{FnValue, Record};

mod user;
use crate::user::user_controller::{add_users, users};

use slog::Drain;
use slog_scope::{info, GlobalLoggerGuard};

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}

fn init_logger() -> GlobalLoggerGuard {
    let drain = slog_json::Json::new(std::io::stdout())
        .set_pretty(true)
        .add_default_keys()
        .add_key_value(slog::o!(
            "file" => FnValue(|rinfo : &Record| {
                rinfo.file()
            }),
            "line" => FnValue(|rinfo : &Record| {
                rinfo.line()
            }),
            "format" => "pretty"
        ))
        .build()
        .fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let log = slog::Logger::root(drain, slog::o!());
    slog_scope::set_global_logger(log)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _guard = init_logger();
    info!("formatted: {}", "1"; "log-key" => true);

    let conn = Database::connect("postgresql://postgres:test@localhost:5432/postgres")
        .await
        .unwrap();
    let state = AppState { conn };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(web::scope("/api/users").service(users).service(add_users))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
