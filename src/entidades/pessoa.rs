use serde::{Deserialize, Serialize};
use crate::entidades::endereco::{Endereco};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Pessoa {
    pub nome: String,
    pub idade: u32,
    pub endereco: Endereco,
    pub cpf: String
}