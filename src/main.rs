#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate rocket;
extern crate rocket_contrib;

mod libs;

use rocket_contrib::json::Json;
use rocket::http::RawStr;
use libs::models::*;
use libs::*;

#[get("/world")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<name>")]
fn hello(name: &RawStr) -> String {
    format!("Hello, {}!", name.as_str())
}

#[derive(FromForm)]
struct Topic{
    url: String
}

#[get("/chat/messages?<topic>")]
fn get_topic(topic: Topic) -> String {
    format!("hello {} !", topic.url)
}

#[get("/chat")]
fn get_chat() -> Json<Vec<Message>> {
    return Json(get_messages());
}

#[get("/chat/<website>")]
fn get_chat_by_website(website : String) -> Json<Vec<Message>> {
    return Json(get_messages_website(website));
}


#[post("/chat", format = "application/json", data = "<message>")]
fn new_message(message: Json<ReceivedMessage>) {
    let connection = establish_connection();

    println!("{:?}", create_message(
            &connection, &message.into_inner()));
}

fn main() {
    rocket::ignite()
        .mount("/hello", routes![index])
        .mount("/hello", routes![hello])
        .mount("", routes![get_topic])
        .mount("", routes![get_chat])
        .mount("", routes![new_message])
        .mount("", routes![get_chat_by_website])
        .launch();
}