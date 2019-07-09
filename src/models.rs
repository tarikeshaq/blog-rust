use uuid::Uuid;


#[derive(Queryable)]
pub struct Post {
    pub id: Uuid,
    pub message: String
}