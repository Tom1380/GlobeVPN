use aws_sdk_ec2::Client;
use std::{fs::OpenOptions, io::prelude::*, os::unix::fs::OpenOptionsExt};

/// If the key pair is not on AWS, create it and save it in GlobeVPN's directory.
pub async fn create_key_pair_if_necessary(client: &Client, key_name: &str) {
    if let Ok(Some(key)) = client
        .create_key_pair()
        .key_name(key_name)
        .send()
        .await
        .map(|k| k.key_material)
    {
        let mut f = OpenOptions::new()
            .write(true)
            .mode(0o600)
            .create(true)
            .open(format!("../{key_name}.pem"))
            .expect("Could not save the generated key to file.");

        write!(f, "{key}").expect("Could not save the generated key to file.");
    }
}
