// TODO put all the AMIs in and change the Option to a &str.
pub const REGION_AMIS: [(&str, Option<&str>); 23] = [
    ("eu-central-1", Some("ami-06e97680b8bf6528e")),
    ("eu-west-1", None),
    ("eu-west-2", Some("ami-061e7843348aee109")),
    ("eu-south-1", None),
    ("eu-west-3", None),
    ("eu-north-1", None),
    ("us-east-1", Some("ami-09b8d4b4d39180649")),
    ("us-east-2", None),
    ("us-west-1", None),
    ("us-west-2", None),
    ("ca-central-1", None),
    ("sa-east-1", Some("ami-0d5990429ef5c2da8")),
    ("ap-southeast-2", Some("ami-081d9e6c6f75745e3")),
    ("ap-east-1", None),
    ("ap-southeast-3", None),
    ("ap-south-1", None),
    ("ap-northeast-3", None),
    ("ap-northeast-2", None),
    ("ap-southeast-1", None),
    ("ap-northeast-1", None),
    ("af-south-1", None),
    ("me-south-1", None),
    ("me-central-1", None),
];

pub fn get_ami(region: &str) -> Option<&str> {
    REGION_AMIS.into_iter().find(|(r, _ami)| r == &region)?.1
}
