use tokio::process::Command;

/// Preshares the authentication key via SSH tunnel.
pub async fn preshare_openvpn_key(ip: &str, key_name: &str) {
    let ssh_key_path = &format!("../{key_name}.pem");

    Command::new("scp")
        .args([
            "-i",
            ssh_key_path,
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
            ssh_key_path,
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

/// Runs the OpenVPN client to connect, the arguments are passed via command line without any configuration file.
pub async fn run_openvpn(ip: &str) {
    let mut c = Command::new("sudo")
        .args([
            "openvpn",
            "--remote",
            ip,
            "--nobind",
            "--dev",
            "tun",
            "--secret",
            "auth.key",
            "--cipher",
            "AES-256-CBC",
            "--ifconfig",
            "10.8.0.1",
            "10.8.0.2",
            "--redirect-gateway",
            "def1",
            "--auth-nocache",
        ])
        .spawn()
        .expect("Couldn't spawn openvpn client.");

    c.wait().await.unwrap();
}
