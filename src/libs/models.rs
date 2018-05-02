use super::schema::messages;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Message {
    pub id: i32,
    pub user_name: String,
    pub message: String,
    pub published: bool,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct ReceivedMessage {
    pub user_name: String,
    pub message: String,
}

#[derive(Insertable)]
#[table_name="messages"]
pub struct NewMessage<'a> {
    pub user_name: &'a str,
    pub message: &'a str,
}