use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ec2::{model::InstanceType, Client, Error, Region};
use std::{fs::OpenOptions, io::Write, os::unix::fs::OpenOptionsExt};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = new_client().await;

    create_key_pair_if_necessary("testiamo", &client).await;
    configure_security_group(&client).await;
    let ip = launch_ec2_instance(&client).await;

    Ok(())
}

async fn new_client() -> Client {
    let region = Region::new("eu-central-1");
    let region_provider = RegionProviderChain::first_try(region)
        .or_default_provider()
        .or_else(Region::new("us-east-2"));

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    Client::new(&shared_config)
}

async fn create_key_pair_if_necessary(desired_name: &str, client: &Client) {
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

async fn configure_security_group(client: &Client) {
    // TODO check if it's failing because the group already exists, or because of other errors.
    // If it already exists, no problem.
    let _ = client
        .create_security_group()
        .group_name("globevpn")
        .description("Security group for GlobeVPN.")
        .send()
        .await;

    // TODO check if it's failing because the rule already exists, or because of other errors.
    // If it already exists, no problem.
    // Note: just because the group already exists doesn't mean it's correctly set,
    // so we shouldn't skip this operation just because the group already exists.
    let _ = client
        .authorize_security_group_ingress()
        .group_name("globevpn")
        .set_ip_protocol(Some("tcp".to_string()))
        .cidr_ip("0.0.0.0/0")
        .from_port(22)
        .to_port(22)
        .send()
        .await;
}

/// Launches a new ec2 instance and returns its public ipv4.
async fn launch_ec2_instance(client: &Client) -> String {
    let r = client
        .run_instances()
        .instance_type(InstanceType::T2Nano)
        .image_id("ami-065deacbcaac64cf2") // ubuntu Frankfurt
        .key_name("testiamo")
        .security_groups("globevpn")
        .min_count(1)
        .max_count(1)
        // .instance_ids("i-0542400d2dc1c0d08")
        .send()
        .await
        .unwrap();

    r.instances
        .unwrap()
        .get(0)
        .unwrap()
        .public_ip_address
        .clone()
        .unwrap()
}
