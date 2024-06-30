use tokio::net::{TcpListener, TcpStream};
use tokio::io::copy;
use tokio::spawn;
use tokio::sync::mpsc;
use std::net::SocketAddr;
use tokio::io;

async fn transfer(
    mut inbound: TcpStream, 
    outbound_address: SocketAddr,
    response_sender: mpsc::Sender<io::Result<()>>,
) {
    match TcpStream::connect(outbound_address).await {
        Ok(mut outbound) => {
            let (mut ri, mut wi) = inbound.split();
            let (mut ro, mut wo) = outbound.split();

            let client_to_proxy = async {

                copy(&mut ri, &mut wo).await
            };

            let proxy_to_client = async {
                copy(&mut ro, &mut wi).await
            };

            let result = tokio::select! {
                res1 = client_to_proxy => res1,
                res2 = proxy_to_client => res2,
            };

            // Convert the Result<u64, Error> to Result<(), Error>
            let result = result.map(|_| ());

            let _ = response_sender.send(result).await;
        },
        Err(e) => {
            let _ = response_sender.send(Err(e)).await;
        }
    }
}

async fn handle_client(inbound: TcpStream) {
    // Forward to Tor running locally on port 9050
    let proxy_addr: SocketAddr = "127.0.0.1:9050".parse().unwrap();
    let (sender, mut receiver) = mpsc::channel(1);
    
    // Log the connection time and client IP
    let timestamp = Utc::now().to_rfc3339();
    let client_ip = inbound.peer_addr().unwrap().ip().to_string();
    log_connection(&timestamp, &client_ip);
    
    spawn(transfer(inbound, proxy_addr, sender));
    if let Some(result) = receiver.recv().await {
        if let Err(e) = result {
            eprintln!("Failed to transfer data: {}", e);
        }
    }
}

fn log_connection(timestamp: &str, client_ip: &str) {
    let log_entry = format!("{} - Connection from {}\n", timestamp, client_ip);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("connections.log")
        .unwrap();
    file.write_all(log_entry.as_bytes()).unwrap();
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // Explicitly define the type of addr to avoid type inference issues
    let addr: SocketAddr = "0.0.0.0:3030".parse().unwrap();
    let listener = TcpListener::bind(addr).await?;

    println!("Listening on: {}", addr);

    loop {
        let (socket, _) = listener.accept().await?;
        spawn(handle_client(socket));
    }
}
