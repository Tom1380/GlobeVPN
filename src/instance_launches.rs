use aws_sdk_ec2::{
    model::{Filter, InstanceType},
    Client,
};

/// Launches a new ec2 instance and returns its public ipv4.
pub async fn launch_ec2_instance(client: &Client) -> String {
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

    std::thread::sleep(std::time::Duration::from_secs(30));

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
