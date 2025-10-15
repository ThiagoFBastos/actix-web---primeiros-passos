use std::sync::RwLock;
use crate::entidades::pessoa::Pessoa;

#[derive(Debug)]
pub struct Database {
    pub pessoas: RwLock<Vec<Pessoa>>
}

impl Database {

    pub fn new() -> Self {
        Self {
            pessoas: RwLock::new(Vec::new())
        }
    }
}