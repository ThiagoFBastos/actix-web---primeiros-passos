use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use validator::Validate;
use dotenv::dotenv;
use std::env;

mod entidades;
mod detail;
mod database;

use entidades::pessoa::Pessoa;
use detail::error::Error;
use database::Database;

#[get("/find/{cpf}")]
async fn find_pessoa(path: web::Path<String>, state: web::Data<Database>) ->  impl Responder {
    let cpf = path.into_inner();
    let db = state.pessoas.read().unwrap();
    let pessoa = db.iter().find(|p| p.cpf == cpf);

    if pessoa.is_none() {
        return HttpResponse::NotFound().json(Error {message: "pessoa não encontrada".to_string()});
    }

    HttpResponse::Ok().json(pessoa.unwrap().clone())
}

#[get("/all")]
async fn get_pessoas(state: web::Data<Database>) -> impl Responder {
    let db = state.pessoas.read().unwrap();
    HttpResponse::Ok().json(db.clone())
}

#[post("/")]
async fn add_pessoa(data: web::Json<Pessoa>, state: web::Data<Database>) -> impl Responder {
    let valido = data.validate();

    if valido.is_err() {
        return HttpResponse::BadRequest().json(valido.unwrap_err());
    }

    let pessoa = data.into_inner();

    let mut db = state.pessoas.write().unwrap();

    if db.iter().any(|p| p.cpf == pessoa.cpf) {
        return HttpResponse::BadRequest().json(Error {message: "cpf já cadastrado".to_string()});
    }

    db.push(pessoa.clone());

    HttpResponse::Created().json(pessoa)
}

#[delete("/delete/{cpf}")]
async fn delete_pessoa(path: web::Path<String>, state: web::Data<Database>) -> impl Responder {
    let cpf = path.into_inner();
    
    let mut db = state.pessoas.write().unwrap();

    let pessoa = db.iter().find(|p| p.cpf == cpf);

    if pessoa.is_none() {
        return HttpResponse::NotFound().json(Error{message: "pessoa não encontrada".to_string()});
    }

    let resultado = pessoa.unwrap().clone();

    db.retain(|p| p.cpf != cpf);

    HttpResponse::Ok().json(resultado)
}

#[put("/update/{cpf}")]
async fn update_pessoa(path: web::Path<String>, data: web::Json<Pessoa>, state: web::Data<Database>) -> impl Responder {
    let valido = data.validate();

    if valido.is_err() {
        return HttpResponse::BadRequest().json(valido.unwrap_err());
    }

    let cpf = path.into_inner();

    let mut db = state.pessoas.write().unwrap();

    if !db.iter().any(|p| p.cpf == cpf) {
        return HttpResponse::NotFound().json(Error{message: "pessoa não encontrada".to_string()});
    } else if data.cpf != cpf {
        return HttpResponse::BadRequest().json(Error{message: "cpf incorreto".to_string()});
    }

   db.retain(|p| p.cpf != cpf);

   let pessoa = data.into_inner();

   db.push(pessoa.clone());

   HttpResponse::Ok().json(pessoa.clone())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = web::Data::new(Database::new());

    dotenv().ok();
    
    let host = env::var("HOST").ok().unwrap();
    let port = env::var("PORT").ok().unwrap().parse::<u16>().expect("falha ao converter");

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .service(find_pessoa)
            .service(get_pessoas)
            .service(add_pessoa)
            .service(delete_pessoa)
            .service(update_pessoa)
    })
    .bind((host, port))?
    .run()
    .await
}