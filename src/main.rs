#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate postgres;
extern crate serde_json;
#[macro_use] 
extern crate serde_derive;
extern crate rocket;
extern crate rocket_contrib;
extern crate chrono;

use chrono::prelude::*;
use postgres::{Connection, TlsMode};

use std::collections::LinkedList;
use rocket_contrib::json::Json;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


#[get("/topic/<topic_name>")]
fn get_topic(topic_name:String) -> String {
    println!("getting topic {}", topic_name);
    let db = get_db(); //todo: replace this madness with a conectionpool (r2d2)

    let query = "SELECT * from messages WHERE topic = $1";
    let mut messages = LinkedList::new();
    for row in &db.query(&query, &[&topic_name]).expect("Failed to select messages"){
        messages.push_back(response_message{
            topic: row.get(0),
            message: row.get(2),
            sender: row.get(3)
            });
    };

    let result = Topic{name:topic_name,messages:messages};

    return serde_json::to_string(&result).expect("failed to parse json");
} 


#[post("/chat", format = "application/json", data = "<recieved_message>")]
fn send_message(recieved_message: Json<recievedMessage>) -> String {
    let db = get_db();  //todo: replace this madness with a conectionpool (r2d2)
    let now = Utc::now();

    let sql = "INSERT INTO messages(topic, message, sender) VALUES($1, $2, $3)";
    let _result = db.execute(sql, &[&recieved_message.topic, &recieved_message.message, &recieved_message.sender])
    .expect("failed to insert message");
    
    return get_topic(recieved_message.topic.clone());

}

fn get_db () -> postgres::Connection{
    let db_url = "psql://digado:digado@localhost:5432/messages";
    let db = Connection::connect(db_url, TlsMode::None)
        .expect("Unable to connect to database");
    return db;
}

#[derive(Serialize, Deserialize)]
struct recievedMessage {
    sender: String,
    message: String,
    topic: String,
}

#[derive(Serialize, Deserialize)]
struct response_message {
    sender: String,
    message: String,
    topic: String,
}


struct db_message {
    sender: String,
    sent: DateTime<Utc>,
    message: String,
    topic: String,
}

#[derive(Serialize, Deserialize)]
struct Topic {
    name: String,
    messages: LinkedList<response_message>,
}





fn main() {


    rocket::ignite()
    .mount("/", routes![index])
    .mount("", routes![get_topic])
    .mount("", routes![send_message])    
    .launch();
    println!("Hello, world!");
    
}


