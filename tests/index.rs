#[macro_use] extern crate trashdove_desktop;
mod index_should_show_icon {
    use rocket::local::blocking::Client;
    use rocket::http::{Status, ContentType};


    fn client() -> Client {
        let client = Client::tracked(super::rocket()).unwrap();
        return client;
    }

    #[test]
    fn test_favicon() {
    }
}