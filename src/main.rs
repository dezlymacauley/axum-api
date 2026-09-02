use tokio::net::TcpListener;
use axum::{Json, Router, extract::Path, routing::get};
use serde::Serialize;

//_____________________________________________________________________________

// SECTION: Helper Functions

// This function will shutdown the server when the user presses `Ctrl C`
async fn listen_for_shutdown_signal() {
    tokio::signal::ctrl_c().await.unwrap();
}

//_____________________________________________________________________________

// SECTION: Route Handlers

async fn root_get() -> String {
    format!("This is the route: /\n")
}

async fn users_get() -> Json<UsersGetReponse> {
    
    let response = UsersGetReponse {

        // Remember to remove the `\n` (newline character 
        // when working with JSON)
        message: format!("This is the route: /users")
    };

    Json(response)
}

async fn users_get_username(Path(username): Path<String>) -> String {

    let username: String = username; 

    let response = format!(
        "This is the route: /users\n\
        The path parameter `username` is: {username}\n"
    );

    response
}

async fn posts_get() -> String {
    format!("This is the route: /posts\n")
}

//_____________________________________________________________________________

// SECTION: Response Structures

// `Serialize` means, convert this Rust code to JSON
#[derive(Serialize)]
struct UsersGetReponse {
    message: String
}

//_____________________________________________________________________________

#[tokio::main]
async fn main() {

    // Creates a TCP listener
    let tcp_listener = TcpListener::bind("127.0.0.1:2986").await.unwrap();

    // Creates an Axum Router with route handlers attached
    let axum_router: Router<()> = Router::new()
        .route("/", get(root_get))
        .route("/users", get(users_get))
        .route("/users/{username}", get(users_get_username))
        .route("/posts", get(posts_get));

    // The data type of server is:
    // Serve<TcpListener, Router, Router>

    // 1. TcpListener
    // This tells axum the network and port where TCP connections 
    // will be comming from.

    // 2. Router, Router
    // This first `Router` is the `axum_router` variable, 
    // for handling requests made to specific endpoints.

    // The second `Router` is axum's built in way to automatically handle
    // requests that don't match any routes.

    let server = axum::serve(tcp_listener, axum_router)
        .with_graceful_shutdown(listen_for_shutdown_signal());

    // Displays the connection information of the server
    println!("\nAttempting to start server at this network address:");
    println!("127.0.0.1:2986\n");

    // Starts the Axum server
    server.await.unwrap();
    
    println!("\nThe server has been shutdown\n");
}
