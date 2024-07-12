mod handshake;
use tokio::time::{timeout, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ip = "52.16.188.185";
    let addr = format!("{}:{}", ip, handshake::DEV_P2P_PORT);

    match timeout(Duration::from_secs(10), handshake::handshake_with_peer(&addr)).await {
        Ok(result) => {
            if let Err(err) = result {
                eprintln!("Handshake failed: {}", err);
            } else {
                println!("Handshake successful!");
            }
        }
        Err(_) => {
            eprintln!("Handshake timed out!");
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_handshake() {
    let ip = "18.218.241.18"; // Actual public Ethereum node IP address
    let addr = format!("{}:{}", ip, handshake::DEV_P2P_PORT);
    let result = handshake::handshake_with_peer(&addr).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_handshake_with_invalid_peer() {
    let ip = "127.0.0.1"; // Invalid IP address
    let addr = format!("{}:{}", ip, handshake::DEV_P2P_PORT);
    let result = handshake::handshake_with_peer(&addr).await;
    assert!(result.is_err());
}
