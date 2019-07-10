use uuid::Uuid;
use serde_derive::{Serialize, Deserialize};
use crate::schema::posts;
use crate::schema::posts::dsl::*;
use crate::establish_connection;
use crate::diesel::prelude::*;


#[derive(Queryable, Serialize, Deserialize, Insertable, Associations, AsChangeset)]
#[table_name = "posts"]
pub struct Post {
    pub id: Uuid,
    pub message: String
}

impl Post {
    pub fn clone(&self) -> Post {
        Post {
            id: self.id,
            message: self.message.clone()
        }
    }
}

pub fn get_all_posts() -> Result<Vec<Post>, diesel::result::Error> {
    let connection = establish_connection();
    posts.load::<Post>(&connection)
}


pub fn get_post(id_to_get: uuid::Uuid) -> Result<Post, String> {
    let connection = establish_connection();
    let value = posts.filter(&id.eq(id_to_get))
                     .load::<Post>(&connection);
    match value {
        Ok(multiple_posts) => {
            if multiple_posts.len() > 0 {
                Ok(multiple_posts[0].clone())
            } else {
                Err(String::from("There are not posts with that id"))
            }
        },
        Err(e) => {
            Err(e.to_string())
        }
    }
}

pub fn create_post(post_to_insert: Post) -> Result<usize, diesel::result::Error> {
    let connection = establish_connection();
    let rows_inserted = crate::diesel::insert_into(posts)
    .values(&post_to_insert)
    .execute(&connection);
    rows_inserted
}

pub fn update_post(id_to_update: uuid::Uuid, post: Post) -> Result<Post, diesel::result::Error> {
    let connection = establish_connection();
    diesel::update(posts.filter(id.eq(id_to_update)))
                .set(&post)
                .get_result(&connection)
}

pub fn delete_post(id_to_delete: uuid::Uuid) -> Result<usize, diesel::result::Error> {
    let connection = establish_connection();
    diesel::delete(posts.filter(id.eq(id_to_delete)))
            .execute(&connection)
}
