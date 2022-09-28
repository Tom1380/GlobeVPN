mod arguments;
mod ec2_instance;
mod key_pairs;
mod manage_directory;
mod openvpn;
mod regions;
mod security_groups;

use self::{
    arguments::{Args, InstanceSize},
    ec2_instance::{get_public_ip, launch_ec2_instance, terminate_ec2_instance},
    key_pairs::create_key_pair_if_necessary,
    manage_directory::change_directory,
    openvpn::{preshare_openvpn_key, run_openvpn},
    regions::{get_region_info, RegionInfo},
    security_groups::configure_security_group,
};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ec2::{model::InstanceType, Client, Error, Region};
use clap::Parser;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let start = Instant::now();

    let args = Args::parse();
    let ri = get_region_info(&args.region);
    let ami = ri
        .ami
        .expect("Couldn't find the AMI for the requested region.");
    let instance_type = get_instance_type(&args, &ri);

    // TODO this is a temporary fix.
    let key_name = &args.region;

    let client = new_client(&args.region).await;

    change_directory(&args.region).await;

    create_key_pair_if_necessary(&client, key_name).await;
    configure_security_group(&client).await;

    let instance_id = launch_ec2_instance(&client, instance_type.clone(), &ami, key_name).await;

    println!(
        "Launched a {:?} EC2 instance in {}.",
        &instance_type, args.region
    );

    let ctrl_c = tokio::spawn(ctrl_c_listener(client.clone(), instance_id.clone()));

    let ip = get_public_ip(&client, &instance_id).await;

    preshare_openvpn_key(&ip, key_name).await;

    println!("Connected in {} seconds.", start.elapsed().as_secs());

    run_openvpn(&ip).await;

    ctrl_c.await.unwrap();

    Ok(())
}

/// Check the arguments and T2 availability in the requested region to determine which instance type to launch.
fn get_instance_type(args: &Args, region_info: &RegionInfo) -> InstanceType {
    use InstanceSize::*;
    // If the user wants T3 explicitly or T2 isn't available, opt for T3.
    if args.t3 || !region_info.has_t2 {
        match args.size {
            Nano => InstanceType::T3Nano,
            Micro => InstanceType::T3Micro,
        }
    } else {
        match args.size {
            Nano => InstanceType::T2Nano,
            Micro => InstanceType::T2Micro,
        }
    }
}

async fn new_client(region: &str) -> Client {
    let region = Region::new(region.to_string());
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
