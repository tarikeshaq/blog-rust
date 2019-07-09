#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{routes, get, post, delete, put};
use rocket;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::json;
use serde_derive::{Serialize, Deserialize};
use uuid::Uuid;
use rocket_contrib;


#[derive(Serialize, Deserialize)]
struct Post {
    id: Uuid,
    message: String
}

#[get("/posts")]
fn posts() ->Json<Vec<Post>> {
    // TODO
    // Get posts from DB
    Json(vec![Post{
        id: uuid::Uuid::new_v4(),
        message: String::from("Hi")
    }])
}

#[get("/post/<id>")]
fn post(id: rocket_contrib::uuid::Uuid) -> Json<Post> {
    // TODO
    // Get post with given id
    Json(Post{
        id: uuid::Uuid::parse_str(&id.into_inner().to_string()).unwrap(),
        message: String::from("It works!")
    })
}

#[post("/post", format = "json", data = "<post>")]
fn new(post: Json<Post>) -> JsonValue {
    // TODO link to diesel and create in db
    json!({"status": "created"})
}

#[delete("/post/<id>")]
fn delete(id: rocket_contrib::uuid::Uuid) -> JsonValue {
    // TODO delete from db with id 
    json!({"status": "deleted", "id": id.into_inner().to_string()})
}

#[put("/post/<id>", format = "json", data = "<post>")]
fn update(id: rocket_contrib::uuid::Uuid, post: Json<Post>) -> Json<Post> {
    // TODO Update value in db
    Json(post.into_inner())
}

fn main() {
    use rust_server::schema::post::dsl::*;
    rocket::ignite().mount("/", routes![posts, post, new, delete, update]).launch();
}