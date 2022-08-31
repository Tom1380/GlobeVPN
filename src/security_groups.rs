use aws_sdk_ec2::Client;


pub async fn configure_security_group(client: &Client) {
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