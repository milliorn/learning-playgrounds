use axum::{routing::get, Json, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // This statement which wires up our router...using new as the name of the function that builds a struct is just convention. Then we have the call to route, as the route function takes mutable ownership of the newly created struct and returns it after it has added the route and the method handler. get is a function that we don't have in our file, it lives in another module, inside the Axum crate.
    let app = Router::new().route("/", get(handler));

    // A server needs to listen for requests therefore we need to create a socket address. We create our socket address using a two-value tuple ([127, 0, 0, 1], 3030)
    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));

    // With the socket address created, we are ready to start our server.
    println!("Server started, listening on {addr}");

    //  We start out by sending an immutable reference of our socket address to the bind function. bind then return a builder where we register our app/router to be served by sending a derived service to serve.  In rust, nothing happens unless something is driving it. To make it ”drive” we need to call await. If we leave await out, the application would never start the server, it would just exit like the main function we got from cargo new. The last thing we do in our program is to call expect.
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}

// Used inside Json as a response because we implemented Serialize using a derive macro:
#[derive(serde::Serialize)]
struct Message {
    message: String,
}

// The -> Json<Message> denotes that the function returns a Json struct which is generic, and that the generic type is Message. For us, this means that it’s a JSON response containing a Message.
async fn handler() -> Json<Message> {
    Json(Message {
        message: String::from("Hello, World!"),
    })
}
