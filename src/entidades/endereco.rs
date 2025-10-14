use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Clone, Debug, Validate)]

pub struct Endereco {
    #[validate(length(min = 8, max = 8, message = "o CEP deve conter 8 digitos"))]
    pub cep: String,
    #[validate(length(min = 3, max = 300, message = "o endereço deve conter entre 3 e 300 caracteres"))]
    pub endereco: String,
    #[validate(length(min = 0, max = 300, message = "o complemento tem no máximo 300 caracteres"))]
    pub complemento: Option<String>
}