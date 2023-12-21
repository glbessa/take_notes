#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::response::Redirect;

#[get("/")]
fn hello() -> String {
    String::from("Hello world!")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello])
}