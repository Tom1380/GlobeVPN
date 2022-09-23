// TODO either change it or add all AMIs.
pub fn get_ami(region: &str) -> Option<String> {
    [
        ("eu-central-1", "ami-06e97680b8bf6528e"),
        ("ap-southeast-2", "ami-081d9e6c6f75745e3"),
        ("sa-east-1", "ami-0d5990429ef5c2da8"),
        ("us-east-1", "ami-09b8d4b4d39180649"),
        ("eu-west-2", "ami-061e7843348aee109"),
    ]
    .into_iter()
    .find(|(r, _ami)| r == &region)
    .map(|(_r, ami)| ami.to_string())
}
