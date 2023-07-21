# Beginning Rust: by exploring a very basic Axum web API in detail

## Source

<https://medium.com/@lindblomdev/beginning-rust-by-exploring-a-very-basic-axum-web-api-in-detail-1f4c87e422e0>

## Dependencies

axum is the web framework we use. We have added it to the dependencies using the shorthand format, which only needs the version.

`axum` is the web framework we use. We have added it to the dependencies using the shorthand format, which only needs the version.

`tokio` is the most popular async runtime, it also happens to be made by the same team that made axum. We use a different format for including tokio than we did axum. The reason is that we need to specify which features we want to enable, and the shorthand format does not have support for that. While name and version are self-explanatory, features might be more diffuse. They allow crate authors to include code that only a subset of the users will need, without negatively impacting the ones that do not need them. If you leave out features, you will just get the default features of the crate. Any feature you add will be added on top of the default features. And you can, if you choose to, opt out of the default features, we will not do that in this series.

`Serde` is and stands for serializing and deserializing. In this article, we will only serialize/deserialize to/from JSON, but if you need any other format in the future, chances are high that there is a crate to make it work with serde. We add the derive feature, more on derive later.

## Launch Backend

`cargo run` in the `backend` folder will start a web server on port 3030, which replies with {“message”:”Hello, World!”} if we hit / with a GET request.
