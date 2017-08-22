#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate rocket_derive;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use rocket::local::Client;
use rocket::http::{ContentType, Status};

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
        assert_eq!(CustomRequest { foo: "Foo".into(), bar: 10 }, request);
        CustomResponse { baz: request.bar }
    }

    let rocket = rocket::ignite().mount("/", routes![handler]);
    let client = Client::new(rocket).unwrap();
    let mut response = client.post("/test")
        .body(serde_json::to_string(&CustomRequest { foo: "Foo".into(), bar: 10 }).unwrap())
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(Status::Ok, response.status());
    let body = response.body_string().expect("No body in test response");
    let response = serde_json::from_str(&*body).expect("Failed to parse response JSON");
    assert_eq!(CustomResponse { baz: 10 }, response);
}
