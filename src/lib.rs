//! Custom derive functionality for [Rocket] applications.
//!
//! ## Usage
//!
//! Add `courier` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! courier = "0.3"
//! serde = "1.0"
//! serde_derive = "1.0"
//! serde_json = "1.0"
//! ```
//!
//! Import the crate and the necessary Serde dependencies into your project:
//!
//! ```rust
//! #[macro_use]
//! extern crate courier;
//! extern crate serde;
//! #[macro_use]
//! extern crate serde_derive;
//! extern crate serde_json;
//! # fn main() {}
//! ```
//!
//! Note that you must have the `#[macro_use]` attribute on the `extern crate` statement in order to
//! use this crate's features.
//!
//! You may now derive [`FromData`] and [`Responder`] for your custom types:
//!
//! ```
//! # #![feature(plugin)]
//! # #![plugin(rocket_codegen)]
//! # extern crate rocket;
//! # #[macro_use] extern crate courier;
//! # extern crate serde;
//! # #[macro_use] extern crate serde_derive;
//! # #[cfg(feature = "json")]
//! # extern crate serde_json;
//! # #[cfg(feature = "msgpack")]
//! # extern crate rmp_serde;
//! #[derive(Deserialize, FromData)]
//! pub struct CustomRequest {
//!     // Some members go here.
//! }
//!
//! #[derive(Serialize, Responder)]
//! pub struct CustomResponse {
//!     // Some members go here.
//! }
//!
//! # fn main() {}
//! ```
//!
//! ## Supported Formats
//!
//! `courier` supports receiving request bodies and sending response bodies in multiple formats.
//! For each one you'd like to enable, you'll have to enable a feature in your `Cargo.toml` and add the
//! relevant Serde crate(s) to your project. The following table shows which formats are currently
//! supported, the feature name for that format, and what Serde crate(s) you'll need to include.
//!
//! | Format        | Feature Name | Serde Crate(s)    |
//! | --------------|--------------|-------------------|
//! | [JSON]        | `json`       | [`serde_json`]    |
//! | [MessagePack] | `msgpack`    | [`rmp-serde`]     |
//!
//! By default, only JSON support is enabled. So, for example, if you'd like to add MessagePack support,
//! you'd edit your `Cargo.toml` to enable the `msgpack` feature and add rmp-serde as a dependency:
//!
//! ```toml
//! [dependencies]
//! rmp-serde = "0.13.6"
//! serde = "1.0"
//! serde_derive = "1.0"
//! serde_json = "1.0"
//!
//! [dependencies.courier]
//! version = "0.3"
//! features = ["msgpack"]
//! ```
//!
//! And then add `rmp-serde` to your project root:
//!
//! ```rust,ignore
//! #[macro_use]
//! extern crate courier;
//!
//! extern crate rmp_serde;
//! extern crate serde;
//! #[macro_use]
//! extern crate serde_derive;
//! extern crate serde_json;
//! # fn main() {}
//! ```
//!
//! Note that, to also support JSON, you still need to include `serde_json` as a dependency. If you do
//! not wish to support JSON, you can specify `default-features = false` in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies.courier]
//! version = "0.3"
//! default-features = false
//! features = ["msgpack"]
//! ```
//!
//! ## Using Multiple Formats
//!
//! When multiple formats are enabled at once, the [`Content-Type`] header in the request is used to
//! determine which format to use. A response will use the same content type specified in the request,
//! so a request sent with JSON will receive a response with JSON, a request sent with MessagePack
//! will get a response with MessagePack, and so on.
//!
//! While this mostly removes the need for [`rocket_contrib::Json`] (and similar types), it is still
//! possible to use it to override the behavior defined with `courier`. For example, say you
//! specify a format for your Rocket route:
//!
//! ```rust
//! # #![feature(plugin)]
//! # #![plugin(rocket_codegen)]
//! # extern crate rocket;
//! # #[macro_use] extern crate courier;
//! # extern crate serde;
//! # #[macro_use] extern crate serde_derive;
//! # #[cfg(feature = "json")]
//! # extern crate serde_json;
//! # #[cfg(feature = "msgpack")]
//! # extern crate rmp_serde;
//! #
//! # #[derive(Deserialize, FromData)]
//! # pub struct CustomRequest {
//! # }
//! #
//! # #[derive(Serialize, Responder)]
//! # pub struct CustomResponse {
//! # }
//! #[post("/endpoint", format = "application/json", data = "<request>")]
//! pub fn handle_request(request: CustomRequest) -> CustomResponse {
//!     CustomResponse {}
//! }
//! # fn main() {}
//! ```
//!
//! In that case, Rocket will check the content type before routing the request to `handle_request`,
//! then the [`FromData`] impl for `CustomRequest` will check it again. If this isn't desirable, you
//! can use [`rocket_contrib::Json`] to skip the second check:
//!
//! ```rust
//! # #![feature(plugin)]
//! # #![plugin(rocket_codegen)]
//! # extern crate rocket;
//! # extern crate rocket_contrib;
//! # #[macro_use] extern crate courier;
//! # extern crate serde;
//! # #[macro_use] extern crate serde_derive;
//! # #[cfg(feature = "json")]
//! # extern crate serde_json;
//! # #[cfg(feature = "msgpack")]
//! # extern crate rmp_serde;
//! #
//! # #[derive(Deserialize, FromData)]
//! # pub struct CustomRequest {
//! # }
//! #
//! # #[derive(Serialize, Responder)]
//! # pub struct CustomResponse {
//! # }
//! use rocket_contrib::Json;
//!
//! #[post("/endpoint", format = "application/json", data = "<request>")]
//! pub fn handle_request(request: Json<CustomRequest>) -> Json<CustomResponse> {
//!     Json(CustomResponse {})
//! }
//! # fn main() {}
//! ```
//!
//! Note, though, that recommended to not explicitly specify the `format` parameter for your route
//! if you're using `courier`. The code generated by `courier` allows you to write
//! content type-agnostic route handlers, so manually specifying an expected format is unnecessary.
//!
//! [Rocket]: https://rocket.rs/
//! [`FromData`]: https://api.rocket.rs/rocket/data/trait.FromData.html
//! [`Responder`]: https://api.rocket.rs/rocket/response/trait.Responder.html
//! [JSON]: http://www.json.org/
//! [MessagePack]: http://msgpack.org/index.html
//! [`serde_json`]: https://crates.io/crates/serde_json
//! [`rmp-serde`]: https://crates.io/crates/rmp-serde
//! [`Content-Type`]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Type
//! [`rocket_contrib::Json`]: https://api.rocket.rs/rocket_contrib/struct.Json.html

#![recursion_limit="128"]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use quote::Tokens;

#[cfg(all(not(feature = "json"), not(feature = "msgpack")))]
compile_error!("No features are enabled, please enable at least one of \"json\", \"msgpack\"");

#[proc_macro_derive(FromData)]
#[doc(hidden)]
pub fn derive_from_data(input: TokenStream) -> TokenStream {
    // Parse the string representation.
    let derive_input = syn::parse_derive_input(&input.to_string()).unwrap();
    let ident = derive_input.ident;

    let json = if cfg!(feature = "json") {
        quote! {{
            let is_json = request.content_type().map(|ct| ct.is_json()).unwrap_or(false);
            if is_json {
                let limit = request.limits().get("json").unwrap_or(u64::MAX);
                let reader = data.open().take(limit);
                return match ::serde_json::from_reader(reader) {
                    Ok(value) => Outcome::Success(value),
                    Err(_error) => Outcome::Failure((Status::InternalServerError, Failure(Status::InternalServerError))),
                };
            }
        }}
    } else {
        Tokens::new()
    };


    let msgpack = if cfg!(feature = "msgpack") {
        quote! {{
            // Accepted content types are:
            //
            // - `application/msgpack`
            // - `application/x-msgpack`
            // - `bin/msgpack`
            // - `bin/x-msgpack`
            let is_msgpack = request.content_type()
                .map(|content_type| {
                    (content_type.top() == "application" || content_type.top() == "bin") &&
                    (content_type.sub() == "msgpack" || content_type.sub() == "x-msgpack")
                })
                .unwrap_or(false);
            if is_msgpack {
                let limit = request.limits().get("msgpack").unwrap_or(u64::MAX);
                let reader = data.open().take(limit);
                return match ::rmp_serde::decode::from_read(reader) {
                    Ok(value) => Outcome::Success(value),
                    Err(_) => Outcome::Failure((
                        Status::InternalServerError,
                        Failure(Status::InternalServerError),
                    )),
                };
            }
        }}
    } else {
        Tokens::new()
    };

    let gen = quote! {
        impl ::rocket::data::FromData for #ident {
            type Error = ::rocket::response::Failure;

            fn from_data(request: &::rocket::Request, data: ::rocket::Data) -> ::rocket::data::Outcome<Self, Self::Error> {
                use rocket::http::Status;
                use rocket::outcome::Outcome;
                use rocket::response::Failure;
                use std::io::Read;
                use std::u64;

                #json

                #msgpack

                ::rocket::outcome::Outcome::Forward(data)
            }
        }
    };
    gen.parse().unwrap()
}

#[proc_macro_derive(Responder)]
#[doc(hidden)]
pub fn derive_responder(input: TokenStream) -> TokenStream {
    // Parse the string representation.
    let derive_input = syn::parse_derive_input(&input.to_string()).unwrap();
    let ident = derive_input.ident;

    let json = if cfg!(feature = "json") {
        quote! {{
            let accept_json = request.accept()
                .map(|accept| accept.preferred().is_json())
                .unwrap_or(false);
            if accept_json {
                return ::serde_json::to_string(&self).map(|string| {
                    ::rocket::response::content::Json(string).respond_to(request).unwrap()
                }).map_err(|_error| {
                    ::rocket::http::Status::InternalServerError
                });
            }
        }}
    } else {
        Tokens::new()
    };

    let msgpack = if cfg!(feature = "msgpack") {
        quote! {{
            // Accepted content types are:
            //
            // - `application/msgpack`
            // - `application/x-msgpack`
            // - `bin/msgpack`
            // - `bin/x-msgpack`
            let accept_msgpack = request.accept()
                .map(|accept| accept.preferred())
                .map(|content_type| {
                    (content_type.top() == "application" || content_type.top() == "bin") &&
                    (content_type.sub() == "msgpack" || content_type.sub() == "x-msgpack")
                })
                .unwrap_or(false);
            if accept_msgpack {
                return rmp_serde::to_vec(&self)
                    .map_err(|_| ::rocket::http::Status::InternalServerError)
                    .and_then(|buf| {
                        ::rocket::response::Response::build()
                            .sized_body(::std::io::Cursor::new(buf))
                            .ok()
                    });
            }
        }}
    } else {
        Tokens::new()
    };

    let gen = quote! {
        /// Serializes the wrapped value into JSON. Returns a response with Content-Type
        /// JSON and a fixed-size body with the serialized value. If serialization
        /// fails, an `Err` of `Status::InternalServerError` is returned.
        impl ::rocket::response::Responder<'static> for #ident {
            fn respond_to(self, request: &::rocket::Request) -> ::rocket::response::Result<'static> {
                #json

                #msgpack

                // If none of the known formats are specified in the `Accept` header, then return
                // a 406 Not Acceptable error to indicate that the resource couldn't be returned
                // in an acceptable format.
                Err(::rocket::http::Status::NotAcceptable)
            }
        }
    };
    gen.parse().unwrap()
}
