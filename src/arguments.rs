use crate::regions::REGIONS;
use aws_sdk_ec2::model::InstanceType;
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, default_value = "eu-central-1", value_parser=REGIONS)]
    pub region: String,

    #[clap(short, long, value_enum, default_value_t=AcceptedInstanceType::T2Micro)]
    pub instance_type: AcceptedInstanceType,
}

#[derive(Debug, Clone, ValueEnum)]
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
