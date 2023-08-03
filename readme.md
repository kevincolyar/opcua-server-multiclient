# OPC UA Server - Multi Client

This is a test server demonstrating how the opcua v0.11 crate is unable to handle more than one connected client. 

## Testing

Start Server

    RUST_LOG=debug cargo run

Start First Client

    RUST_LOG=debug cargo run --bin client

Start Second Client

    RUST_LOG=debug cargo run --bin client
