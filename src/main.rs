pub mod pages;

use crate::pages::shared_input;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
    .mount("/", routes![index])
    .mount("/shared_input", routes![shared_input::index])
    .launch()
    .await?;

    Ok(())
}