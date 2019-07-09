#[macro_use] extern crate diesel;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use models::*;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn getAllPosts() -> Vec<Post> {
    use schema::post::dsl::*;

    let connection = establish_connection();
    post.limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts")
}