use tokio::net::TcpListener;
use axum::{Router, routing::get};

//_____________________________________________________________________________

// SECTION: Route Handlers

async fn root_get() -> String {
    format!("This is the route: /\n")
}

async fn users_get() -> String {
    format!("This is the route: /users\n")
}

async fn posts_get() -> String {
    format!("This is the route: /posts\n")
}

//_____________________________________________________________________________

#[tokio::main]
async fn main() {

    // Creates a TCP listener
    let tcp_listener = TcpListener::bind("127.0.0.1:2986").await.unwrap();

    // Creates an Axum Router with route handlers attached.
    let axum_router: Router<()> = Router::new()
        .route("/", get(root_get))
        .route("/users", get(users_get))
        .route("/posts", get(posts_get));

    // The data type of server is:
    // Serve<TcpListener, Router, Router>

    // 1. TcpListener
    // This tells axum the network and port where TCP connections 
    // will be comming from.

    // 2. Router, Router
    // This first Router is the user defined service for handling routes 
    // that you defined.

    // The second Router is axum's built in way to automatically handle
    // requests that don't match any routes.

    let server = axum::serve(tcp_listener, axum_router);

    // Displays the connection information of the server
    println!("\nAttempting to start server at this network address:");
    println!("127.0.0.1:2986\n");

    // Starts the Axum server
    server.await.unwrap();
}
