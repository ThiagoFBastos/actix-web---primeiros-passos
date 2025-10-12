use actix_web::{delete, error, get, post, put, web, App, HttpResponse, HttpServer, Responder, Result};
use std::sync::RwLock;

mod entidades;

use entidades::{pessoa::Pessoa};

#[derive(Debug)]
struct Database {
    pessoas: RwLock<Vec<Pessoa>>
}

#[get("/find/{cpf}")]
async fn find_pessoa(path: web::Path<String>, state: web::Data<Database>) ->  Result<impl Responder> {
    let cpf = path.into_inner();
    let db = state.pessoas.read().unwrap();
    let pessoa = db.iter().find(|p| p.cpf == cpf);

    if pessoa.is_none() {
        return Err(error::ErrorNotFound("pessoa não encontrada"));
    }

    Ok(web::Json(pessoa.unwrap().clone()))
}

#[get("/all")]
async fn get_pessoas(state: web::Data<Database>) -> Result<impl Responder> {
    let db = state.pessoas.read().unwrap();
    Ok(web::Json(db.clone()))
}

#[post("/")]
async fn add_pessoa(path: web::Json<Pessoa>, state: web::Data<Database>) -> Result<impl Responder> {
    let pessoa = path.into_inner();

    let mut db = state.pessoas.write().unwrap();

    if db.iter().any(|p| p.cpf == pessoa.cpf) {
        return Err(error::ErrorBadRequest("cpf já cadastrado"));
    }

    db.push(pessoa.clone());

    Ok(HttpResponse::Created().json(pessoa))
}

#[delete("/delete/{cpf}")]
async fn delete_pessoa(path: web::Path<String>, state: web::Data<Database>) -> Result<impl Responder> {
    let cpf = path.into_inner();
    
    let mut db = state.pessoas.write().unwrap();

    if !db.iter().any(|p| p.cpf == cpf) {
        return Err(error::ErrorNotFound("pessoa não encontrada"));
    }

    db.retain(|p| p.cpf != cpf);

    Ok(web::Json(cpf))
}

#[put("/update/{cpf}")]
async fn update_pessoa(path: web::Path<String>, data: web::Json<Pessoa>, state: web::Data<Database>) -> Result<impl Responder>{
    let cpf = path.into_inner();

    let mut db = state.pessoas.write().unwrap();

    if !db.iter().any(|p| p.cpf == cpf) {
        return Err(error::ErrorNotFound("pessoa não encontrada"));
    } else if data.cpf != cpf {
        return Err(error::ErrorBadRequest("cpf incorreto"));
    }

   db.retain(|p| p.cpf != cpf);

   let pessoa = data.into_inner();

   db.push(pessoa.clone());

   Ok(web::Json(pessoa))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = web::Data::new(Database {
        pessoas: RwLock::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .service(find_pessoa)
            .service(get_pessoas)
            .service(add_pessoa)
            .service(delete_pessoa)
            .service(update_pessoa)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}