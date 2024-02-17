#[macro_use]
extern crate rocket;
mod db;
mod routers;
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
#[launch]
fn app() -> _ {
    rocket::build().mount("/", routes![index])
}
