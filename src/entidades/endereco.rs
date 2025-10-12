use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
#[derive(Debug)]
pub struct Endereco {
    pub cep: String,
    pub endereco: String,
    pub complemento: Option<String>,
}