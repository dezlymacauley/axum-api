use tokio::net::TcpListener;
use axum::Router;

#[tokio::main]
async fn main() {

    // Creates a TCP listener
    let listener = TcpListener::bind("127.0.0.1:2986").await.unwrap();

    let app: Router<()> = Router::new();

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

    let server = axum::serve(listener, app);

    // Displays the connection information of the server
    println!("\nAttempting to start server at this network address:");
    println!("127.0.0.1:2986\n");

    // Starts the Axum server
    server.await.unwrap();
}
