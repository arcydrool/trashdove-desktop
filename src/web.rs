
use rocket::response::Redirect;
use rocket::{Build, Rocket}; 
use rocket::fs::{relative, FileServer};
#[get("/")]
fn index() -> Redirect {
    return  Redirect::to("/index.html");
}

pub fn mount(rocket_builder: Rocket<Build>) -> Rocket<Build> {
    rocket_builder.mount("/", routes![index]).mount(
        "/",
        FileServer::new(relative!("td"), rocket::fs::Options::None),
    )
}