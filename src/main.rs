#[macro_use] extern crate rocket;
use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Message {
    content: String,
}

#[post("/", format = "json", data = "<message>")]
fn index(message: Json<Message>) -> Json<Message> {
    Json(Message {
        content: format!("Received: {}", message.content),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

