use aws_sdk_ec2::Client;
use std::{fs::OpenOptions, io::prelude::*, os::unix::fs::OpenOptionsExt};
use uuid::Uuid;

/// Creates a new key pair on AWS, saves the private key to file and returns its name.
pub async fn create_new_key_pair(client: &Client) -> String {
    let key_name = format!("globevpn-{}", Uuid::new_v4());

    if let Ok(Some(key)) = client
        .create_key_pair()
        .key_name(&key_name)
        .send()
        .await
        .map(|k| k.key_material)
    {
        let mut f = OpenOptions::new()
            .write(true)
            .mode(0o600)
            .create(true)
            .open(format!("{key_name}.pem"))
            .expect("Could not save the generated key to file.");

        write!(f, "{key}").expect("Could not save the generated key to file.");

        return key_name;
    } else {
        println!("Could not generate a new key pair.");
        std::process::exit(1);
    }
}
