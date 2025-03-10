#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

//#[get("/favicon.ico")]
//TODO: FAVICON, style.css, war?
// https://github.com/rwf2/Rocket/blob/master/examples/static-files/src/main.rs

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}