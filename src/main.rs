use apistonks::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")
        .expect("Failed to bind random port");
    
    // midly useless
    let port = listener.local_addr().unwrap().port();
    println!("http://127.0.0.1:{}", port);

    run(listener)?.await
}