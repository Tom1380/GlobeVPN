// TODO put all the AMIs in and change the Option to a &str.
pub const REGION_INFO: [RegionInfo; 23] = [
    RegionInfo::new("eu-central-1", Some("ami-0163e22b06e9d08d4"), true),
    RegionInfo::new("eu-west-1", None, true),
    RegionInfo::new("eu-west-2", None, true),
    RegionInfo::new("eu-south-1", Some("ami-09412495bdfcff6e0"), false),
    RegionInfo::new("eu-west-3", None, true),
    RegionInfo::new("eu-north-1", None, true),
    RegionInfo::new("us-east-1", None, true),
    RegionInfo::new("us-east-2", None, true),
    RegionInfo::new("us-west-1", None, true),
    RegionInfo::new("us-west-2", None, true),
    RegionInfo::new("ca-central-1", None, true),
    RegionInfo::new("sa-east-1", None, true),
    RegionInfo::new("ap-southeast-2", None, true),
    RegionInfo::new("ap-east-1", None, true),
    RegionInfo::new("ap-southeast-3", None, true),
    RegionInfo::new("ap-south-1", None, true),
    RegionInfo::new("ap-northeast-3", None, true),
    RegionInfo::new("ap-northeast-2", None, true),
    RegionInfo::new("ap-southeast-1", None, true),
    RegionInfo::new("ap-northeast-1", None, true),
    RegionInfo::new("af-south-1", None, true),
    RegionInfo::new("me-south-1", None, true),
    RegionInfo::new("me-central-1", None, true),
];

pub fn get_region_info(region: &str) -> RegionInfo {
    REGION_INFO
        .into_iter()
        .find(|info| info.region == region)
        // Shouldn't happen as clap ensures that the region is in REGION_INFO.
        .expect("Couldn't find specified region.")
}

pub struct RegionInfo<'a> {
    pub region: &'a str,
    pub ami: Option<&'a str>,
    pub has_t2: bool,
}

impl<'a> RegionInfo<'a> {
    const fn new(region: &'a str, ami: Option<&'a str>, has_t2: bool) -> Self {
        Self {
            region,
            ami,
            has_t2,
        }
    }
}
