use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::models::Todo;



pub type TodoStore = Arc<Mutex<HashMap<String, Todo>>>;

pub fn create_store() -> TodoStore {
    Arc::new(Mutex::new(HashMap::new()))
}

