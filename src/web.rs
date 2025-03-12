
use rocket::response::content;
use rocket::{Build, Rocket}; 
use rocket::fs::{relative, FileServer};
#[get("/")]
fn index() -> content::RawHtml<&'static str> {
    content::RawHtml(
        r#"
    <html>
    <head></head>
    <body>text</body>
    </html>
    "#,
    )
}

pub fn mount(rocket_builder: Rocket<Build>) -> Rocket<Build> {
    rocket_builder.mount("/", routes![index]).mount(
        "/",
        FileServer::new(relative!("td"), rocket::fs::Options::None),
    )
}