use tokio::process::Command;

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
