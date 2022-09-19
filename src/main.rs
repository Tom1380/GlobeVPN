mod ec2_instance;
mod key_pairs;
mod manage_directory;
mod openvpn;
mod security_groups;

use self::{
    ec2_instance::{launch_ec2_instance, setup_ec2_instance},
    key_pairs::create_key_pair_if_necessary,
    manage_directory::change_directory,
    openvpn::run_openvpn,
    security_groups::configure_security_group,
};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ec2::{Client, Error, Region};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = new_client().await;

    change_directory("eu-central-1").await;

    create_key_pair_if_necessary("globevpn3", &client).await;
    configure_security_group(&client).await;
    let ip = launch_ec2_instance(&client).await;

    std::thread::sleep(std::time::Duration::from_secs(30));

    setup_ec2_instance(&ip).await;

    std::thread::sleep(std::time::Duration::from_secs(5));

    run_openvpn(&ip).await;
    Ok(())
}

async fn new_client() -> Client {
    let region = Region::new("eu-central-1");
    let region_provider = RegionProviderChain::first_try(region);

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    Client::new(&shared_config)
}
