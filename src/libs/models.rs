use super::schema::messages;
use rocket_contrib::Json;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Message {
    pub id: i32,
    pub user_name: String,
    pub message: String,
    pub published: bool,
}

#[derive(Insertable)]
#[table_name="messages"]
pub struct NewMessage<'a> {
    pub user_name: &'a str,
    pub message: &'a str,
}