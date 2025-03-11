#[macro_use]
extern crate rocket;
#[cfg(test)]
mod tests;

//use rocket::{Rocket, Request, Build};
use rocket::response::content; //, status};
                               //use rocket::http::Status;
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

#[rocket::launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index]).mount(
        "/",
        FileServer::new(relative!("td"), rocket::fs::Options::None),
    )
}
