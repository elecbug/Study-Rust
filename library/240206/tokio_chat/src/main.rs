use std::net::SocketAddrV4;
use std::env;
use std::io;

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncWriteExt, AsyncReadExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let max_id: i32 = args[1].parse().unwrap();
    
    let (listener, id) = create_listener().await;

    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer)?;
    println!("Started full dense connections...");

    let mut clients = create_clients(id, max_id).await;

    loop {
        io::stdin().read_line(&mut buffer)?;
        let strs: Vec<&str> = buffer.split(" ").collect::<Vec<&str>>();

        if strs.len() != 2 {
            println!("Invalid command");
            continue;
        }

        let id: i32 = match strs[0].parse() {
            Ok(id) => {
                if let Err(e) = clients[id as usize].write_all(strs[1].as_bytes()).await {
                    println!("Message is not sended");
                };

                id
            },
            Err(e) => {
                println!("Invalid ID");
                continue;
            }
        };
    }

    Ok(())
}

async fn create_clients(id: i32, range: i32) -> Vec<TcpStream> {
    let mut clients = Vec::new();

    for i in 1..=range {
        if i == id {
            continue;
        }

        clients.push(match TcpStream::connect(create_ip(i)).await {
            Ok(o) => {
                println!("{}: {:?}", i, o);
                o
            },
            Err(e) => {
                println!("Do not alive {}", i);
                continue;
            },
        });
    }

    clients
}

async fn create_listener() -> (TcpListener, i32) {
    let mut num = 1;

    loop {
        match TcpListener::bind(create_ip(num)).await {
            Ok(o) => {
                println!("Host from {}", create_ip(num));
                return (o, num);
            },
            Err(_e) => {
                num += 1;
                continue;
            }
        };
    }
}

fn create_ip(id: i32) -> SocketAddrV4 {
    format!("127.0.0.{}:12356", id).parse().unwrap()
}