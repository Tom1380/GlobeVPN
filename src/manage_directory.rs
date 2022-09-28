use dirs::home_dir;
use std::{
    env, fs,
    path::{Path, PathBuf},
};
use tokio::process::Command;
use uuid::Uuid;

const MAIN_DIRECTORY: &str = ".globevpn";

/// Returns the path of GlobeVPN's operating directory.
pub fn globevpn_directory_path() -> PathBuf {
    home_dir()
        .unwrap_or_else(|| {
            println!("Couldn't find your home directory.");
            std::process::exit(1);
        })
        .join(PathBuf::from(MAIN_DIRECTORY))
}

/// Creates a new directory, moves into it, saves a new OpenVPN key in it and returns the directory's path.
pub async fn change_directory(region: &str) -> PathBuf {
    let uuid = Uuid::new_v4();

    let specific_path = globevpn_directory_path().join(Path::new(&format!("{region}-{uuid}")));

    fs::create_dir_all(&specific_path).expect("Could not create {specific_path} directory");

    env::set_current_dir(&specific_path).unwrap();

    generate_openvpn_key().await;

    specific_path
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
