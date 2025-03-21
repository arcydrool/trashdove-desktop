use rocket::fs::{relative, FileServer};
use rocket::response::Redirect;
use rocket::{Build, Rocket};

#[get("/")]
fn index() -> Redirect {
    return Redirect::to("/index.html");
}

#[get("/id")]
fn id() -> String {
    return uuid::Uuid::new_v4().as_u128().to_string();
}

pub fn mount(rocket_builder: Rocket<Build>) -> Rocket<Build> {
    rocket_builder
        //tricked me - mount id to / to get /id. mounting id to /id gets you /id/id
        .mount("/", routes![id])
        .mount("/", routes![index])
        .mount(
            "/",
            FileServer::new(relative!("td"), rocket::fs::Options::None),
        )
}
