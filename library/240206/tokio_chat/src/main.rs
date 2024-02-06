use tokio::net::{TcpListener, TcpStream};
use tokio::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut start = 1;

    let mut listener: TcpListener;
    let mut id = 0;

    loop {
        listener = match TcpListener::bind(create_ip(start)).await {
            Ok(l) => {
                println!("Host from {}", create_ip(start));
                id = start;
                l
            },
            Err(e) => {
                start += 1;
                continue;
            }
        };
    }

    Ok(())
}

fn create_ip(id: i32) -> String {
    String::from(format!("127.0.0.{}:12356", id))
}