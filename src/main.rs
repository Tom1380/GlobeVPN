mod ec2_instance;
mod key_pairs;
mod manage_directory;
mod openvpn;
mod security_groups;

use self::{
    ec2_instance::{get_public_ip, launch_ec2_instance, terminate_ec2_instance},
    key_pairs::create_key_pair_if_necessary,
    manage_directory::change_directory,
    openvpn::{preshare_openvpn_key, run_openvpn},
    security_groups::configure_security_group,
};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ec2::{Client, Error, Region};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let key_name = "new3";

    let client = new_client().await;

    change_directory("eu-central-1").await;

    create_key_pair_if_necessary(&client, key_name).await;
    configure_security_group(&client).await;

    let instance_id = launch_ec2_instance(&client, key_name).await;

    println!("Launched EC2 instance.");

    let ctrl_c = tokio::spawn(ctrl_c_listener(client.clone(), instance_id.clone()));

    let ip = get_public_ip(&client, &instance_id).await;

    preshare_openvpn_key(&ip, key_name).await;

    run_openvpn(&ip).await;

    ctrl_c.await.unwrap();

    Ok(())
}

async fn new_client() -> Client {
    let region = Region::new("eu-central-1");
    let region_provider = RegionProviderChain::first_try(region);

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    Client::new(&shared_config)
}

async fn ctrl_c_listener(client: Client, instance_id: String) {
    tokio::signal::ctrl_c().await.unwrap();
    println!("Received Ctrl+C!");
    terminate_ec2_instance(&client, &instance_id).await;
    println!("Terminated the instance.");

    std::process::exit(0);
}
