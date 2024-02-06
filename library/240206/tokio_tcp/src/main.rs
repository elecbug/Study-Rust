use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    
    loop {
        let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
        let (mut socket, _) = listener.accept().await?;
        
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            let mut count = 0;

            loop {
                if let Err(e) = stream.write_all(b"Hello, world").await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }

                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => {
                        let s = match std::str::from_utf8(&buf) {
                            Ok(v) => v,
                            Err(e) => {
                                eprintln!("failed convert text; err = {:?}", e);
                                return;
                            },
                        };

                        count += 1;

                        if (count % 1000 == 0) {
                            println!("{}", s);
                            println!("count: {}", count);
                        }

                        continue;
                    },
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    },
                };

                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}