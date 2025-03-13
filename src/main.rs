#[macro_use] extern crate rocket;
use rocket::serde::{json::{Json, Value}, Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct Message {
    content: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/json")]
fn json_re() -> Json<Value> {
    Json(json!({ "message": "json message" }))
}

#[post("/", format = "json", data = "<message>")]
fn post_message(message: Json<Message>) -> Json<Message> {
    Json(Message {
        content: format!("Received: {}", message.content)
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, json_re, post_message])
}

