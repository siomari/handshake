use tokio::net::TcpStream;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use rlp::RlpStream;
use rand::Rng;

pub const DEV_P2P_PORT: u16 = 30303;

struct HelloMessage {
    protocol_version: u8,
    client_id: String,
    capabilities: Vec<(String, u8)>,
    listen_port: u16,
    node_id: [u8; 64],
}

impl HelloMessage {
    fn encode(&self) -> Vec<u8> {
        let mut stream = RlpStream::new_list(5);
        stream.append(&self.protocol_version);
        stream.append(&self.client_id);
        stream.append_raw(&encode_capabilities(&self.capabilities), 1);
        stream.append(&self.listen_port);
        stream.append(&self.node_id.to_vec());
        stream.out().to_vec()
    }
}

fn encode_capabilities(capabilities: &[(String, u8)]) -> Vec<u8> {
    let mut stream = RlpStream::new_list(capabilities.len());
    for (name, version) in capabilities {
        stream.begin_list(2);
        stream.append(name);
        stream.append(version);
    }
    stream.out().to_vec()
}

pub async fn handshake_with_peer(addr: &str) -> io::Result<()> {
    let mut stream = TcpStream::connect(addr).await?;

    let mut node_id = [0u8; 64];
    rand::thread_rng().fill(&mut node_id);
    let hello_msg = HelloMessage {
        protocol_version: 5,
        client_id: "rust-devp2p/0.1".to_string(),
        capabilities: vec![("eth".to_string(), 63)],
        listen_port: 0,
        node_id,
    };

    let encoded_msg = hello_msg.encode();
    stream.write_all(&encoded_msg).await?;

    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;

    println!("Received: {:?}", &buffer[..n]);

    Ok(())
}
