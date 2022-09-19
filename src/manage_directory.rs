use std::{
    env, fs,
    path::{Path, PathBuf},
};
use tokio::process::Command;
use uuid::Uuid;

const MAIN_DIRECTORY: &str = ".globevpn";

/// Returns the path of GlobeVPN's operating directory.
pub fn globevpn_directory_path() -> PathBuf {
    PathBuf::from(MAIN_DIRECTORY)
}

pub async fn change_directory(region: &str) {
    let uuid = Uuid::new_v4();

    let specific_path = globevpn_directory_path().join(Path::new(&format!("{region}-{uuid}")));

    fs::create_dir_all(&specific_path).expect("Could not create {specific_path} directory");

    env::set_current_dir(&specific_path).unwrap();

    generate_openvpn_key().await;
}

async fn generate_openvpn_key() {
    let mut c = Command::new("openvpn")
        .args([
            "--genkey",
            "--secret",
            "auth.key",
            "--cipher",
            "AES-256-CBC",
        ])
        .spawn()
        .expect("Couldn't generate a new secret OpenVPN key.");

    c.wait().await.unwrap();
}
