use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]

pub struct Endereco {
    pub cep: String,
    pub endereco: String,
    pub complemento: Option<String>
}