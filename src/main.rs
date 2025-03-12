#[macro_use] extern crate rocket;
use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Message {
    content: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/", format = "json", data = "<message>")]
fn post_message(message: Json<Message>) -> Json<Message> {
    Json(Message {
        content: format!("Received: {}", message.content),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, post_message])
}
