use crate::ami::REGION_AMIS;
use aws_sdk_ec2::model::InstanceType;
use clap::{Parser, ValueEnum};

/// Extract an array of regions only from the array of region-ami tuples.
const fn regions() -> [&'static str; REGION_AMIS.len()] {
    let mut regions = [""; REGION_AMIS.len()];

    let mut i = 0;

    // Circumnavigates the ban on for loops in constant functions.
    while i < regions.len() {
        regions[i] = REGION_AMIS[i].0;
        i += 1;
    }

    regions
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, default_value = "eu-central-1", value_parser=regions())]
    #[clap(help = "Where you will appear from when establishing connections.")]
    pub region: String,

    #[clap(short, long, value_enum, default_value_t=AcceptedInstanceType::T2Micro)]
    #[clap(
        help = "Choose which instance type to spin up. T2Nano is cheaper, but AWS offers 750 free hours of T2Micro per month for the first year."
    )]
    pub instance_type: AcceptedInstanceType,
}

#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum AcceptedInstanceType {
    T2Nano,
    T2Micro,
}

impl Into<InstanceType> for AcceptedInstanceType {
    fn into(self) -> InstanceType {
        use InstanceType::*;
        match self {
            Self::T2Nano => T2Nano,
            Self::T2Micro => T2Micro,
        }
    }
}
