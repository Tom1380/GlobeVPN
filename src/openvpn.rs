use std::{process::Stdio, time::Duration};
use tokio::{process::Command, time::sleep};

/// Preshares the authentication key via SSH tunnel.
pub async fn preshare_openvpn_key(ip: &str, key_name: &str) {
    let ssh_key_path = &format!("../{key_name}.pem");

    for _ in 0..30 {
        let exit_status = Command::new("scp")
            .args([
                "-i",
                ssh_key_path,
                "-o",
                "StrictHostKeyChecking no",
                "auth.key",
                &format!("ubuntu@{ip}:/home/ubuntu"),
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Couldn't generate a new secret OpenVPN key.")
            .wait()
            .await
            .unwrap();

        if exit_status.success() {
            break;
        }

        sleep(Duration::from_secs(2)).await;
    }

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
        .stdout(Stdio::null())
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
