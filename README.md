# P2P Node Handshake

In this project a basic handshake protocol for the Ethereum network has been implemented in Rust.

It aims to perform a protocol-level handshake with a publicly available Ethereum node.

## Handshake Protocol

The protocol follows the Ethereum specifications

1. Establish a TCP connection to the node.

2. Send a `Hello` message, which contains:
- Protocol version
- Client Id
- Capabilities
- Port
- Node Id

3. Then read the response to complete the handshake.
