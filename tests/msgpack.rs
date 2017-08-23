#![cfg(feature = "msgpack")]

#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate courier;
extern crate rmp_serde;
extern crate serde;
#[macro_use]
extern crate serde_derive;

#[cfg(feature = "json")]
extern crate serde_json;

use rocket::local::Client;
use rocket::http::{Accept, ContentType, MediaType, Status};

#[derive(Debug, Serialize, Deserialize, FromData, PartialEq, Eq)]
pub struct CustomRequest {
    pub foo: String,
    pub bar: usize,
}

#[derive(Debug, Serialize, Deserialize, Responder, PartialEq, Eq)]
pub struct CustomResponse {
    pub baz: usize,
}

#[test]
fn request_response() {
    #[post("/test", data = "<request>")]
    fn handler(request: CustomRequest) -> CustomResponse {
        println!("Received request: {:?}", request);
        assert_eq!(CustomRequest { foo: "Foo".into(), bar: 10 }, request);
        CustomResponse { baz: request.bar }
    }

    let rocket = rocket::ignite().mount("/", routes![handler]);
    let client = Client::new(rocket).unwrap();
    let mut response = client.post("/test")
        .body(rmp_serde::to_vec(&CustomRequest { foo: "Foo".into(), bar: 10 }).unwrap())
        .header(ContentType::new("application", "msgpack"))
        .header(Accept::new(&[MediaType::new("application", "msgpack").into()]))
        .dispatch();
    assert_eq!(Status::Ok, response.status());
    let body = response.body_bytes().expect("No body in test response");
    let response = rmp_serde::from_slice(&*body).expect("Failed to parse response JSON");
    assert_eq!(CustomResponse { baz: 10 }, response);
}

#[test]
fn not_acceptable() {
    #[post("/test", data = "<request>")]
    fn handler(request: CustomRequest) -> CustomResponse {
        assert_eq!(CustomRequest { foo: "Foo".into(), bar: 10 }, request);
        CustomResponse { baz: request.bar }
    }

    let rocket = rocket::ignite().mount("/", routes![handler]);
    let client = Client::new(rocket).unwrap();
    let response = client.post("/test")
        .body(rmp_serde::to_vec(&CustomRequest { foo: "Foo".into(), bar: 10 }).unwrap())
        .header(ContentType::new("application", "msgpack"))
        .dispatch();
    assert_eq!(Status::NotAcceptable, response.status());
}
