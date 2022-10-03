use aws_sdk_ec2::{
    model::{Filter, InstanceType},
    Client,
};
use std::time::Duration;
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
    for _ in 0..60 {
        match try_get_public_ip(client, instance_id).await {
            Some(ip) => {
                println!("The instance's IP is {ip}.");
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

/// Terminates an EC2 instance.
pub async fn terminate_ec2_instance(client: Client, instance_id: String) {
    match client
        .terminate_instances()
        .instance_ids(&instance_id)
        .send()
        .await
    {
        Ok(_) => println!("Terminated the instance."),
        Err(e) => {
            println!("Couldn't terminate EC2 instance with ID {instance_id}! Do it manually to avoid wrong billings.\n{:?}", e)
        }
    }
}

/// Deletes the SSH key pair.
pub async fn delete_ec2_key_pair(client: Client, key_name: String) {
    match client.delete_key_pair().key_name(&key_name).send().await {
        Ok(_) => println!("Deleted key pair from AWS."),
        Err(e) => println!("Couldn't delete key pair {key_name} from AWS!\n{:?}", e),
    }
}
