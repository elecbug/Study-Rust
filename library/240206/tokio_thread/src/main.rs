use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut count = 0;
    let (tx, mut rx) = mpsc::channel(32);

    let t = tokio::spawn(async move {
        loop {
            if count % 10000 == 0 {
                println!("Thread A: {count}");
            }

            count += 1;

            if count > 1000000 {
                tx.send(count).await;
                return;
            }
        }
    });

    let _ = t.await;

    if let c = rx.recv().await {
        println!("{}", c.ok_or("It's not print format")?);
    };


    Ok(())
}