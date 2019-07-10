#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{routes, get, post, delete, put};
use rocket;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::json;
use rocket_contrib;
use rust_server::models::*;

#[get("/posts")]
fn posts() ->Option<Json<Vec<Post>>> {

    match get_all_posts() {
        Ok(posts) => Some(Json(posts)),
        // TODO Add error handling
        Err(_e) => None
    }
}

#[get("/post/<id>")]
fn post(id: rocket_contrib::uuid::Uuid) -> Option<Json<Post>> {
    match get_post(uuid::Uuid::parse_str(&id.into_inner().to_string()).unwrap()) {
        Ok(post) => Some(Json(post)),
        Err(_e) => None
    }
}

#[post("/post", format = "json", data = "<post>")]
fn new(post: Json<Post>) -> JsonValue {
    let rows_inserted = create_post(post.into_inner());
    match rows_inserted {
        Ok(_val) => json!({
            "status": "created"
        }),
        Err(e) => json!({
            "error" : e.to_string()
        })
    }
}

#[delete("/post/<id>")]
fn delete(id: rocket_contrib::uuid::Uuid) -> JsonValue {
    let deleted = delete_post(uuid::Uuid::parse_str(&id.to_string()).unwrap());
    match deleted {
        Ok(_val) => json!({"status": "deleted", "id": id.into_inner().to_string()}),
        Err(e) => json!({"status": "could not delete", "error": e.to_string()})
    }
    
}

#[put("/post/<id>", format = "json", data = "<post>")]
fn update(id: rocket_contrib::uuid::Uuid, post: Json<Post>) -> Option<Json<Post>> {
    let updated_successfully = update_post(uuid::Uuid::parse_str(&id.to_string()).unwrap(), post.into_inner());
    match updated_successfully {
        Ok(post_updated) => Some(Json(post_updated)),
        // TODO add error handling
        Err(_e) => None
    } 
}

fn main() {
    rocket::ignite().mount("/", routes![posts, post, new, delete, update]).launch();
}