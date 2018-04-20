#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

use rocket::request::FromForm;
use rocket::http::RawStr;

extern crate rocket;

extern crate serde_derive;

extern crate serde;
extern crate serde_json;

const CHAT: &'static [&'static str] = &["hello", "no u", "stop"];

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

#[get("/chat/messages")]
fn write_chat() -> String {
    return serde_json::to_string(CHAT).unwrap();
}

#[post("/chat/message")]
fn send_message() {

}

fn main() {
    rocket::ignite()
        .mount("/hello", routes![index])
        .mount("/hello", routes![hello])
        .mount("/api", routes![get_topic])
        .mount("/api", routes![write_chat])
        .launch();
}