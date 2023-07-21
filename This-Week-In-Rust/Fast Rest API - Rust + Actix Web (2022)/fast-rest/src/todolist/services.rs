use actix_web::{get, post, put, delete, web, Responder, HttpResponse};
use crate::{AppState, TodolistEntry};
use super::models::{CreateEntryData, UpdateEntryData};

/* housing for our endpoints */

/* return all entries */
#[get("/todolist/entries")]
async fn get_entries(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(data.todolist_entries.lock().unwrap().to_vec())
}

/* add entry to list */
#[post("/todolist/entries")]
async fn create_entry(data: web::Data<AppState>, param_obj: web::Json<CreateEntryData>) -> impl Responder {
    /* extract our entries from list */
    let mut todolist_entries = data.todolist_entries.lock().unwrap();
    /* we have to handle our id incrementing */
    let mut max_id: i32 = 0;
    
    /* loop entries and find id and store it */
    for i in 0..todolist_entries.len() {
        if todolist_entries[i].id > max_id {
            max_id = todolist_entries[i].id;
        }
    }

    /* create new todolist entry*/
    todolist_entries.push(TodolistEntry {
        id: max_id + 1,
        title: param_obj.title.clone(),
        date: param_obj.date
    });

    /* return updated list of todolist entries */
    HttpResponse::Ok().json(todolist_entries.to_vec())
}

/* edit entry */
#[put("/todolist/entries/{id}")]
async fn update_entry(data: web::Data<AppState>, path: web::Path<i32>, param_obj: web::Json<UpdateEntryData>) -> impl Responder {
  /* access id in the path */
    let id = path.into_inner();
    /* access entries */
    let mut todolist_entries = data.todolist_entries.lock().unwrap();
    
    /* iterate entries, if we find match then we update title */
    for i in 0..todolist_entries.len() {
        if todolist_entries[i].id == id {
            todolist_entries[i].title = param_obj.title.clone();
            break;
        }
    }

    HttpResponse::Ok().json(todolist_entries.to_vec())
}

/* allow user to delete entry*/
#[delete("/todolist/entries/{id}")]
async fn delete_entry(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let mut todolist_entries = data.todolist_entries.lock().unwrap();
    let id = path.into_inner();

    /* filter existing todolist entries, set that filter vector into app state */
    *todolist_entries = todolist_entries.to_vec().into_iter().filter(|x| x.id != id).collect();

    HttpResponse::Ok().json(todolist_entries.to_vec())
}

/* pass server functions to web server */
pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(get_entries).service(create_entry).service(update_entry).service(delete_entry);
}