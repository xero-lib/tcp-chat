use tracing::{error, info, trace, warn};
use tokio::net::TcpListener;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init(); // for logging uwu
    info!("Ready to explode");

    let listener =
        TcpListener::bind(&std::env::var("LISTENER").expect("env var 'LISTENER' not set"))
            .await
            .expect("Failed to bind tcp listener");

    let (send_tx, mut recv_rc) = tokio::sync::mpsc::unbounded_channel::<Vec<u8>>();
    let (bc_tx, _) = tokio::sync::broadcast::channel(128);

    let bc_tx = std::sync::Arc::new(bc_tx);
    let bc_tx2 = bc_tx.clone();
    
    tokio::spawn(async move {
        while let Some(msg) = recv_rc.recv().await {
            bc_tx.send(msg).unwrap();
        }
    });

    while let Ok((stream, peer)) = listener.accept().await {
        info!("Got connection from peer {:?}", peer);

        let send_tx = send_tx.clone();
        let mut bc_rc = bc_tx2.subscribe();
        tokio::spawn(async move {
            let (mut read, mut write) = stream.into_split();
            tokio::spawn(async move {
                while let Ok(msg) = bc_rc.recv().await {
                    write.write_all(msg.as_ref()).await;
                }
            });
            let mut buffer = [0u8; 256];
            while let Ok(amount) = read.read(&mut buffer[..]).await {
                if amount == 0 {
                    break; // We hit EOF
                }
                send_tx.send(Vec::from(&buffer[..amount])).unwrap();
            }
        });
    }
}

// https://doc.rust-lang.org/stable/std/future/trait.Future.html