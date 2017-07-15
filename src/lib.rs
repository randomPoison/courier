#![recursion_limit="128"]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(FromData)]
pub fn derive_from_data(input: TokenStream) -> TokenStream {
    // Parse the string representation.
    let derive_input = syn::parse_derive_input(&input.to_string()).unwrap();
    let ident = derive_input.ident;

    let gen = quote! {
        impl ::rocket::data::FromData for #ident {
            type Error = ::serde_json::error::Error;

            fn from_data(request: &::rocket::Request, data: ::rocket::Data) -> ::rocket::data::Outcome<Self, Self::Error> {
                use std::io::Read;

                // Maximum size of JSON is 1MB.
                // TODO: Determine this size from some configuration parameter.
                // TODO: Why do we need a maximum size anyway?
                const MAX_SIZE: u64 = 1048576;

                let is_json = request.content_type().map(|ct| ct.is_json()).unwrap_or(false);
                if !is_json {
                    return ::rocket::outcome::Outcome::Forward(data);
                }

                let reader = data.open().take(MAX_SIZE);
                match ::serde_json::from_reader(reader) {
                    Ok(value) => ::rocket::outcome::Outcome::Success(value),
                    Err(e) => ::rocket::outcome::Outcome::Failure((Status::BadRequest, e)),
                }
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
                ::serde_json::to_string(&self).map(|string| {
                    ::rocket::response::content::JSON(string).respond().unwrap()
                }).map_err(|_error| {
                    ::rocket::http::Status::InternalServerError
                })
            }
        }
    };
    gen.parse().unwrap()
}
