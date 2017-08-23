# Changelog

## Courier v0.3.1 (2017-08-23)

Quick update to the v0.3 release fixing some issues with the initial version.

### Fixes

* Fixed the generated [`Responder`] impl to actually generate MessagePack responses when the
  `msgpack` feature is enabled, and to not generate JSON responses when the `json` feature is
  disabled. ([#1](https://github.com/excaliburHisSheath/courier/pull/1))
* Fixed the generated [`Responder`] impl to check the [`Accept`] header to determine what format
  to use for the response body. ([#1](https://github.com/excaliburHisSheath/courier/pull/1))

## Courier v0.3 (2017-08-22)

Initial release supporting v0.3.x of Rocket. Provides support for deriving [`FromData`] and
[`Responder`] for any types that implement [`serde::Deserialize`] and [`serde::Serialize`],
respectively. Also supports using both [JSON] and [MessagePack] formats for data transfer.

[Rocket]: https://rocket.rs/
[`FromData`]: https://api.rocket.rs/rocket/data/trait.FromData.html
[`Responder`]: https://api.rocket.rs/rocket/response/trait.Responder.html
[JSON]: http://www.json.org/
[MessagePack]: http://msgpack.org/index.html
[`serde::Deserialize`]: https://docs.rs/serde/1/serde/trait.Deserialize.html
[`serde::Serialize`]: https://docs.rs/serde/1/serde/trait.Serialize.html
[`Accept`]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Accept
