#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate rocket_derive;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Deserialize, FromData)]
pub struct CustomRequest {
    pub foo: String,
    pub bar: usize,
}

#[derive(Serialize, Responder)]
pub struct CustomResponse {
    pub baz: usize,
}

#[post("/endpoint", data = "<request>")]
pub fn handle_request(request: CustomRequest) -> CustomResponse {
    if request.foo == "foo" {
        CustomResponse { baz: 0 }
    } else {
        CustomResponse { baz: request.bar }
    }
}

fn main() {
    rocket::ignite().mount("/", routes![handle_request]).launch();
}
