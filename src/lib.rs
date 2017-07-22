#![feature(compile_error)]
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
pub fn derive_from_data(input: TokenStream) -> TokenStream {
    // Parse the string representation.
    let derive_input = syn::parse_derive_input(&input.to_string()).unwrap();
    let ident = derive_input.ident;

    let json = from_data_json();
    let msgpack = from_data_msgpack();

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
pub fn derive_responder(input: TokenStream) -> TokenStream {
    // Parse the string representation.
    let derive_input = syn::parse_derive_input(&input.to_string()).unwrap();
    let ident = derive_input.ident;

    let gen = quote! {
        /// Serializes the wrapped value into JSON. Returns a response with Content-Type
        /// JSON and a fixed-size body with the serialized value. If serialization
        /// fails, an `Err` of `Status::InternalServerError` is returned.
        impl ::rocket::response::Responder<'static> for #ident {
            fn respond_to(self, request: &::rocket::Request) -> ::rocket::response::Result<'static> {
                // TODO: Serialize to different formats based on the content type of the request.
                ::serde_json::to_string(&self).map(|string| {
                    ::rocket::response::content::Json(string).respond_to(request).unwrap()
                }).map_err(|_error| {
                    ::rocket::http::Status::InternalServerError
                })
            }
        }
    };
    gen.parse().unwrap()
}

fn from_data_json() -> Tokens {
    if cfg!(feature = "json") {
        quote! {{
            // todo: support different content types.
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
    }
}

fn from_data_msgpack() -> Tokens {
    if cfg!(feature = "msgpack") {
        quote! {{
            // Accepted content types are:
            //
            // - `application/msgpack`
            // - `application/x-msgpack`,
            // - `bin/msgpack`
            // - `bin/x-msgpack`.
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
                    Err(_error) => Outcome::Failure((Status::InternalServerError, Failure(Status::InternalServerError))),
                };
            }
        }}
    } else {
        Tokens::new()
    }
}
