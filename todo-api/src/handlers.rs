use std::f32::consts::PI;

use axum::{extract::{Path, State}, http::StatusCode, Json};

use crate::{errors::AppError, models::{CreateTodo, Todo, UpdateTodo}, state::TodoStore};

const TITLE_CANNOT_BE_EMPTY: &str = "Title cannot be empty";

pub async fn get_todos(State(store): State<TodoStore>) -> Result<Json<Vec<Todo>>, AppError> {
    let todos = store.lock().unwrap();
    let todo_list: Vec<Todo> = todos.values().cloned().collect();
    Ok(Json(todo_list))
}

pub async fn get_todo(Path(id): Path<String>, State(store): State<TodoStore>) -> Result<Json<Todo>, AppError> {
    let todos = store.lock().unwrap();
    match todos.get(&id){
        Some(todo) => Ok(Json(todo.clone())),
        None => Err(AppError::TodoNotFound)
    }
}

pub async fn create_todo(
    State(store): State<TodoStore>,
    Json(payload): Json<CreateTodo>) -> Result<(StatusCode, Json<Todo>), AppError> 
{
    if payload.title.trim().is_empty() {
        return Err(AppError::InvalidInput(TITLE_CANNOT_BE_EMPTY.to_string()));
    }

    let todo = Todo::new(payload.title, Some(payload.description));
    
    let mut todos = store.lock().unwrap();
    todos.insert(todo.id.clone(), todo.clone());

    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn update_todo(
    State(store): State<TodoStore>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateTodo>
) -> Result<Json<Todo>, AppError> {
    let mut todos = store.lock().unwrap();

    match todos.get_mut(&id) {
        Some(todo) => {
            if let Some(ref title) = payload.title {
                if (title.trim().is_empty()) {
                    return Err(AppError::InvalidInput(TITLE_CANNOT_BE_EMPTY.to_string()))
                }
            }
            todo.update(payload);
            Ok(Json(todo.clone()))
        }
        None => Err(AppError::TodoNotFound)
    }
}

pub async fn delete_todo(
    Path(id): Path<String>,
    State(store): State<TodoStore>
) -> Result<StatusCode, AppError> {
    let mut todos = store.lock().unwrap();

    match todos.remove(&id) {
        Some(_) => Ok(StatusCode::OK),
        None => Err(AppError::TodoNotFound)
    }
}