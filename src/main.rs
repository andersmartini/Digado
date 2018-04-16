#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

use rocket::request::FromForm;

extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
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

#[get("/api/chat/messages?<topic>")]
fn get_topic(topic: Topic) -> String {
    format!("hello {} !", topic.url)
}


fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/", routes![get_topic])
        .launch();
}