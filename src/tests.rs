use std::fs::File;
use std::io::Read;

use rocket::local::blocking::Client;
use rocket::http::Status;

use super::rocket;

#[track_caller]
fn test_query_file<T> (path: &str, file: T, status: Status)
    where T: Into<Option<&'static str>>
{
    let client = Client::tracked(rocket()).unwrap();
    let response = client.get(path).dispatch();
    assert_eq!(response.status(), status);

    let body_data = response.into_bytes();
    if let Some(filename) = file.into() {
        let expected_data = read_file_content(filename).expect(filename);
        assert!(body_data.map_or(false, |s| s == expected_data));
    }
}

fn read_file_content(path: &str) -> std::io::Result<Vec<u8>> {
    let mut file_content = vec![];
    let mut fp = File::open(path)?;
    fp.read_to_end(&mut file_content)?;
    Ok(file_content)
}

#[test]
fn test_short_path() {
    test_query_file("/",  None, Status::SeeOther);
    test_query_file("/?v=1", None,Status::SeeOther);
    test_query_file("/?this=should&be=ignored",None,Status::SeeOther);
}

#[test]
fn test_index_html() {
    test_query_file("/index.html", "td/index.html", Status::Ok);
    test_query_file("/index.html?v=1", "td/index.html", Status::Ok);
    test_query_file("/index.html?this=should&be=ignored", "td/index.html", Status::Ok);
}

#[test]
fn test_icon_file() {
    test_query_file("/favicon.ico", "td/favicon.ico", Status::Ok);
}

#[test]
fn test_invalid_path() {
    test_query_file("/thou_shalt_not_exist", None, Status::NotFound);
    test_query_file("/thou/shalt/not/exist", None, Status::NotFound);
    test_query_file("/thou/shalt/not/exist?a=b&c=d", None, Status::NotFound);
}

#[test]
fn test_leaky_path() {
    test_query_file("/../Cargo.toml", None,  Status::NotFound);
}