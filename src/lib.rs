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

pub fn get_all_posts() -> Vec<Post> {
    use schema::posts::dsl::*;

    let connection = establish_connection();
    posts.limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts")
}

pub fn create_post(post_to_insert: Post) -> Result<usize, diesel::result::Error> {
    let connection = establish_connection();
    use schema::posts::dsl::*;
    let rows_inserted = diesel::insert_into(posts)
    .values(&post_to_insert)
    .execute(&connection);
    rows_inserted
}