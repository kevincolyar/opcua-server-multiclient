use opcua::server::prelude::*;

fn main() {
    tracing_subscriber::fmt::init();

    let user_token_ids = vec![ANONYMOUS_USER_TOKEN_ID.to_string()];

    let server = 
        ServerBuilder::new()
        .application_name("my opcua demo server")
        .application_uri("urn:OPC UA Sample Server")
        .host_and_port("0.0.0.0", 4840)
        .receive_buffer_size(131072)
        .send_buffer_size(131072)
        .endpoint("none", ServerEndpoint::new_none("/", &user_token_ids))
        .discovery_urls(vec!["/".into()])
        .server()
        .unwrap();

    server.run();
}
