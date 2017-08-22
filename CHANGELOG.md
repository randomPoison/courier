# Changelog

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
