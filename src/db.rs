use scylla::Session;
use std::env;

// ScyllaDB initialize function.
// Get contact points from environment variables and connect.
pub async fn init() -> Session {
    let contact_points = match env::var("SCYLLA.CONTACT_POINTS") {
        Ok(contact_points) => contact_points,
        Err(_) => {
            println!("Error loading env info for ScyllaDB connection");
            panic!("Error loading env variables to connect to ScyllaDB");
        }
    };

    let session = scylla::SessionBuilder::new()
        .known_nodes(&vec![contact_points])
        .build()
        .await.expect("Error connecting to ScyllaDB cluster");

    session
}