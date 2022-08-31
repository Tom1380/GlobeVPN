mod instance_launches;
mod key_pairs;
mod openvpn;
mod security_groups;

use self::{
    instance_launches::launch_ec2_instance,
    key_pairs::create_key_pair_if_necessary,
    openvpn::{generate_openvpn_configuration, run_openvpn},
    security_groups::configure_security_group,
};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ec2::{Client, Error, Region};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = new_client().await;

    create_key_pair_if_necessary("20agosto", &client).await;
    configure_security_group(&client).await;
    let ip = launch_ec2_instance(&client).await;
    let path = generate_openvpn_configuration(&ip);

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
