use aws_sdk_ec2::Client;
use std::{fs::OpenOptions, io::prelude::*, os::unix::fs::OpenOptionsExt};

pub async fn create_key_pair_if_necessary(desired_name: &str, client: &Client) {
    if let Ok(Some(key)) = client
        .create_key_pair()
        .key_name(desired_name)
        .send()
        .await
        .map(|k| k.key_material)
    {
        let mut f = OpenOptions::new()
            .write(true)
            .mode(0o600)
            .create(true)
            .open(format!("{desired_name}.pem"))
            .expect("Could not save the generated key to file.");

        write!(f, "{key}").expect("Could not save the generated key to file.");
    }
}
