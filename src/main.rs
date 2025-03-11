#[macro_use]
extern crate rocket;
#[cfg(test)]
mod tests;

pub mod web;

#[rocket::launch]
fn rocket() -> _ {
    web::mount(rocket::build())
}
