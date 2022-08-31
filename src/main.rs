use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ec2::{
    model::{Filter, InstanceType},
    Client, Error, Region,
};
use std::{fs::OpenOptions, io::prelude::*, os::unix::fs::OpenOptionsExt, process::Command};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = new_client().await;

    create_key_pair_if_necessary("20agosto", &client).await;
    configure_security_group(&client).await;
    let ip = launch_ec2_instance(&client).await;
    let path = generate_configuration(&ip);

    std::thread::sleep(std::time::Duration::from_secs(30));

    run_openvpn(path);

    println!("Done, hopefully");

    Ok(())
}

async fn new_client() -> Client {
    let region = Region::new("eu-west-2");
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

    let cidr = public_ip::addr_v4()
        .await
        .map(|ip| format!("{ip}/32"))
        .unwrap_or_else(|| {
            println!("Couldn't retrieve your public ip address.");
            println!("Enabling all IP address, be wary.");
            "0.0.0.0/0".to_string()
        });

    // TODO check if it's failing because the rule already exists, or because of other errors.
    // If it already exists, no problem.
    // Note: just because the group already exists doesn't mean it's correctly set,
    // so we shouldn't skip this operation just because the group already exists.
    let _ = client
        .authorize_security_group_ingress()
        .group_name("globevpn")
        .ip_protocol("tcp")
        .cidr_ip(&cidr)
        .from_port(22)
        .to_port(22)
        .send()
        .await;

    let _ = client
        .authorize_security_group_ingress()
        .group_name("globevpn")
        .ip_protocol("udp")
        .cidr_ip(&cidr)
        .from_port(1194)
        .to_port(1194)
        .send()
        .await;
}

/// Launches a new ec2 instance and returns its public ipv4.
async fn launch_ec2_instance(client: &Client) -> String {
    let run_instances_output = client
        .run_instances()
        .instance_type(InstanceType::T2Nano)
        .image_id("ami-08247070f19f16c8f")
        .key_name("20agosto")
        .security_groups("globevpn")
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .unwrap();
    let instance_id = run_instances_output
        .instances()
        .unwrap()
        .get(0)
        .unwrap()
        .instance_id()
        .unwrap();

    std::thread::sleep(std::time::Duration::from_secs(60));

    get_public_ip(&client, instance_id).await
}

async fn get_public_ip(client: &Client, instance_id: &str) -> String {
    client
        .describe_network_interfaces()
        .filters(
            Filter::builder()
                .name("attachment.instance-id")
                .values(instance_id)
                .build(),
        )
        .send()
        .await
        .unwrap()
        .network_interfaces
        .unwrap()
        .get(0)
        .unwrap()
        .association
        .as_ref()
        .unwrap()
        .public_ip
        .as_ref()
        .unwrap()
        .to_string()
}

/// Generates a config file, saves it and returns its path.
fn generate_configuration(ip: &str) -> &str {
    let template = include_str!("template.ovpn");

    let config = template.replace("{ip}", ip);

    let path = "uk.ovpn";
    std::fs::write(path, config).expect("Could not save the template.");

    path
}

/// Runs the OpenVPN client to connect.
fn run_openvpn(path: &str) {
    Command::new("sudo")
        .args(["openvpn", "--config", path])
        .spawn()
        .expect("Couldn't spawn openvpn client.");
}
