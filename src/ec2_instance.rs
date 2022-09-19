use aws_sdk_ec2::{
    model::{Filter, InstanceType},
    Client,
};
use std::time::{Duration, Instant};
use tokio::{process::Command, time::sleep};

/// Launches a new ec2 instance and returns its public ipv4.
pub async fn launch_ec2_instance(client: &Client) -> String {
    let run_instances_output = client
        .run_instances()
        // TODO let this be choosable by the user.
        .instance_type(InstanceType::T2Micro)
        .image_id("ami-06e97680b8bf6528e")
        .key_name("globevpn3")
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

    get_public_ip(&client, instance_id).await
}

async fn get_public_ip(client: &Client, instance_id: &str) -> String {
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

/// Preshares the authentication key via SSH tunnel.
pub async fn setup_ec2_instance(ip: &str) {
    Command::new("scp")
        .args([
            "-i",
            "../globevpn3.pem",
            "-o",
            "StrictHostKeyChecking no",
            "auth.key",
            &format!("ubuntu@{ip}:/home/ubuntu"),
        ])
        .spawn()
        .expect("Couldn't generate a new secret OpenVPN key.")
        .wait()
        .await
        .unwrap();

    Command::new("ssh")
        .args([
            "-i",
            "../globevpn3.pem",
            "-o",
            "StrictHostKeyChecking no",
            &format!("ubuntu@{ip}"),
            "-t",
            "sudo mv auth.key /etc/openvpn/server",
        ])
        .spawn()
        .expect("Couldn't generate a new secret OpenVPN key.")
        .wait()
        .await
        .unwrap();
}
