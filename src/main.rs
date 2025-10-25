use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod entidades;
mod detail;
mod routes;
mod database;

use routes::pessoa::*;
use database::Database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = web::Data::new(Database::new());

    dotenv().ok();
    
    let host = env::var("HOST").ok().unwrap();
    let port = env::var("PORT").ok().unwrap().parse::<u16>().expect("falha ao converter");

    HttpServer::new(move || {
        App::new().service(
            web::scope("/api")
                .app_data(db.clone())
                .service(find_pessoa)
                .service(get_pessoas)
                .service(add_pessoa)
                .service(delete_pessoa)
                .service(update_pessoa)
        )
    })
    .bind((host, port))?
    .run()
    .await
}