use aws_sdk_ec2::Client;

pub async fn configure_security_group(client: &Client) {
    let _ = client
        .create_security_group()
        .group_name("globevpn")
        .description("Security group for GlobeVPN.")
        .send()
        .await;

    // TODO check if it's failing because the rule already exists, or because of other errors.
    // If it already exists, no problem.
    // Note: just because the group already exists doesn't mean it's correctly set,
    // so we shouldn't skip this operation just because the group already exists.
    let _ = client
        .authorize_security_group_ingress()
        .group_name("globevpn")
        .ip_protocol("tcp")
        .cidr_ip("0.0.0.0/0")
        .from_port(22)
        .to_port(22)
        .send()
        .await;

    let _ = client
        .authorize_security_group_ingress()
        .group_name("globevpn")
        .ip_protocol("udp")
        .cidr_ip("0.0.0.0/0")
        .from_port(1194)
        .to_port(1194)
        .send()
        .await;
}
