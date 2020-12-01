use crate::database;
use std::sync::Mutex;

#[derive(Debug)]
pub struct Context {
    pub db: Mutex<database::Database>,
}

impl juniper::Context for Context {}
