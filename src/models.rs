use uuid::Uuid;
use serde_derive::{Serialize, Deserialize};
use crate::schema::posts;


#[derive(Queryable, Serialize, Deserialize, Insertable, Associations)]
#[table_name = "posts"]
pub struct Post {
    pub id: Uuid,
    pub message: String
}