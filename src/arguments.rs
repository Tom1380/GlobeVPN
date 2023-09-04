use crate::regions::REGION_INFO;
use clap::{Parser, ValueEnum};

/// Extract an array of regions only from the array of region-ami tuples.
const fn regions() -> [&'static str; REGION_INFO.len()] {
    let mut regions = [""; REGION_INFO.len()];

    let mut i = 0;

    // Works around the ban on for loops in constant functions.
    while i < regions.len() {
        regions[i] = REGION_INFO[i].region;
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

    #[clap(short, long, value_enum, default_value_t=InstanceSize::Micro)]
    #[clap(
        help = "Choose which instance size to spin up. Nano is cheaper, but AWS offers 750 free hours per month of the lowest available generation of Micro for the first year."
    )]
    pub size: InstanceSize,

    #[clap(long, action)]
    #[clap(
        help = "Chooses T3 over T2 even when both are available. Keep in mind that AWS' offer only applies to the lowest available generation of Micro."
    )]
    pub t3: bool,
}

#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum InstanceSize {
    Nano,
    Micro,
}
