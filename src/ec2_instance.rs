use aws_sdk_ec2::{
    model::{Filter, InstanceType},
    Client,
};
use std::time::{Duration, Instant};
use tokio::time::sleep;

/// Launches a new EC2 instance and returns its instance ID.
pub async fn launch_ec2_instance(
    client: &Client,
    instance_type: InstanceType,
    ami: &str,
    key_name: &str,
) -> String {
    let run_instances_output = client
        .run_instances()
        .instance_type(instance_type)
        .image_id(ami)
        .key_name(key_name)
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

    instance_id.to_string()
}

/// Retrieves the EC2 instance's public IPv4 address.
pub async fn get_public_ip(client: &Client, instance_id: &str) -> String {
    let instant = Instant::now();

    for _ in 0..60 {
        match try_get_public_ip(client, instance_id).await {
            Some(ip) => {
                println!("Got public IP address after {:?}", instant.elapsed());
                return ip;
            }
            None => sleep(Duration::from_secs(2)).await,
        }
    }

    panic!("Couldn't retrieve the instance's public IP address.");
}

async fn try_get_public_ip(client: &Client, instance_id: &str) -> Option<String> {
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
        .ok()?
        .network_interfaces?
        .get(0)?
        .association
        .to_owned()?
        .public_ip
}

pub async fn terminate_ec2_instance(client: &Client, instance_id: &str) {
    client
        .terminate_instances()
        .instance_ids(instance_id)
        .send()
        .await
        .expect("Couldn't terminate EC2 instance. Do it manually to avoid wrong billings.");
}
