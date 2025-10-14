use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::entidades::endereco::{Endereco};

#[derive(Serialize, Deserialize, Clone, Debug, Validate)]
pub struct Pessoa {
    #[validate(length(min = 2, max = 100, message = "o nome deve conter entre 2 e 100 caracteres"))]
    pub nome: String,
    #[validate(range(min = 0, max = 125, message = "a idade tem que ser entre 0 e 125"))]
    pub idade: u32,
    #[validate(nested)]
    pub endereco: Endereco,
    #[validate(length(min = 11, max = 11, message = "o CPF deve conter 11 caracteres"))]
    pub cpf: String
}