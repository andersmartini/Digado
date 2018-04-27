#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;

extern crate dotenv;
extern crate rocket;
extern crate rocket_contrib;

mod libs;

use rocket::request::FromForm;
use rocket::http::RawStr;
use rocket_contrib::json::Json;

use libs::*;
use libs::models::Message;

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
#[derive(FromForm)]
struct Subject{
    url: String,
    sam: String
}

#[get("/chat/messages?<topic>")]
fn get_topic(topic: Topic) -> String {
    format!("hello {} !", topic.url)
}

#[get("/chat")]
fn get_chat() -> Json<Vec<Message>> {
    return Json(get_messages());
}

/*#[post("/chat", format = "application/json", data = "<message>")]
fn new_message(input: Json<Message>) {
}*/

fn main() {
    rocket::ignite()
        .mount("/hello", routes![index])
        .mount("/hello", routes![hello])
        .mount("/api", routes![get_topic])
        .mount("/api", routes![get_chat])
        .launch();
}