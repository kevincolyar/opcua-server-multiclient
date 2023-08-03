use std::{thread::sleep, time::Duration};

use opcua::client::prelude::*;

fn main() {
    tracing_subscriber::fmt::init();

    let opc_url = &"opc.tcp://0.0.0.0:4840/";

    // Make the client configuration
    let mut client = ClientBuilder::new()
        .application_name("Simple Client")
        .application_uri("urn:SimpleClient")
        .product_uri("urn:SimpleClient")
        .trust_server_certs(true)
        .create_sample_keypair(true)
        .session_retry_limit(3)
        .client()
        .unwrap();

    if let Ok(session) = client.connect_to_endpoint(
        (
            opc_url.as_ref(),
            SecurityPolicy::None.to_str(),
            MessageSecurityMode::None,
            UserTokenPolicy::anonymous(),
        ),
        IdentityToken::Anonymous,
    ){
        loop {

            let session = session.write();

            let nodes_to_browse = browse_description(ObjectId::ObjectsFolder.into());

            walk_nodes(&session, nodes_to_browse);

            sleep(Duration::from_secs(4));
        }
    }
}


// Define browse parameters to start from the root node and include all node classes
fn browse_description(node_id: NodeId) -> BrowseDescription {
    BrowseDescription {
        node_id,
        browse_direction: BrowseDirection::Forward,
        reference_type_id: ReferenceTypeId::HierarchicalReferences.into(),
        include_subtypes: true,
        node_class_mask: 0xFFFFFFFF,
        result_mask: 0x3F,
    }
}

fn read_node(session: &Session, reference_desc: ReferenceDescription) { 
    let node_id = reference_desc.node_id.node_id;
    let nodes = vec![node_id.clone().into()];
    let read_response = session.read(&nodes, TimestampsToReturn::Both, 0.0).unwrap();

    let result = read_response[0].clone();
    let value = result.value;

    println!("{:?}: {:?}", node_id.identifier.to_string().replace("s=", ""), value);
}

fn walk_nodes(session: &Session, bd: BrowseDescription) {
    // Browse the server's nodes starting from the root node
    let browse_response = session.browse(&[bd]).unwrap().unwrap();

    // Iterate over the browse results and read the value of each node
    for browse_result in browse_response {
        for reference_desc in browse_result.references.unwrap() {
            // Read the value of the node if it has a value attribute
            if reference_desc.node_class == NodeClass::Variable {
                read_node(&session, reference_desc);
            }
            else if reference_desc.node_id.node_id.namespace == 2 {
                let bd2 = browse_description(reference_desc.node_id.node_id);
                walk_nodes(session, bd2);
            }
        }
    }
}
