extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

use self::models::{Message, NewMessage};

pub fn create_message<'a>(conn: &PgConnection, user_name: &'a str, message: &'a str) -> Message {
    use schema::messages;

    let new_message = NewMessage {
        user_name: user_name,
        message: message,
    };

    diesel::insert_into(messages::table)
        .values(&new_message)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn get_messages () -> Vec<Message> {
    use self::schema::messages::dsl::*;

    let connection = establish_connection();
    let results = messages.filter(published.eq(true))
        .limit(5)
        .load::<Message>(&connection)
        .expect("Error loading posts");
    return results;
}