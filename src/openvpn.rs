use std::process::Command;

/// Generates an OpenVPN config file, writes it to disk and returns its path.
pub fn generate_openvpn_configuration(ip: &str) -> &str {
    let template = include_str!("template.ovpn");

    let config = template.replace("{ip}", ip);

    let path = "uk.ovpn";
    std::fs::write(path, config).expect("Could not save the template.");

    path
}

/// Runs the OpenVPN client to connect.
pub fn run_openvpn(path: &str) {
    let mut c = Command::new("sudo")
        .args(["openvpn", "--config", path])
        .spawn()
        .expect("Couldn't spawn openvpn client.");

    c.wait().unwrap();
}
