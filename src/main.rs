use clap::{Parser, ValueEnum};
use itertools::Itertools;
use machineid_rs::{Encryption, HWIDComponent, IdBuilder};
use std::path::{Path, PathBuf};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// Generate a 'device ID' that can be used to uniquely identify a computer based on files or hardware configuration.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional key to be used as encryption key
    // A tribute to Scatman John
    #[arg(
        short,
        long,
        default_value = "Ski-bi da ba dibby du da ba dibby [?], do I'm the Scatman"
    )]
    key: String,

    /// Sets a file that the content will be used on encryption
    #[arg(short, long, value_parser=file_exists)]
    token_file: Option<PathBuf>,

    /// Parts to be used on ID generation, check 'everything' to use all parts
    #[arg(short, long, value_parser, num_args = 1.., value_delimiter = ' ', default_value="system-id")]
    parts: Vec<UserHWIDComponent>,

    /// Use all parts
    #[arg(long, conflicts_with = "parts")]
    everything: bool,
}

fn file_exists(input: &str) -> Result<PathBuf, String> {
    let path = Path::new(input);
    if !path.is_file() {
        return Err(format!("\'{input}\' is not a file or does not exist."));
    }
    Err("File token is not yet fully supported!".into())
    // Ok(path.into())
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Hash, EnumIter)]
pub enum UserHWIDComponent {
    /// System UUID
    SystemID,
    /// Number of CPU Cores
    CPUCores,
    /// Name of the OS
    OSName,
    /// Current Username
    Username,
    /// Host machine name
    MachineName,
    /// Mac Address
    MacAddress,
    /// CPU Vendor ID
    CPUID,
    /// UUID of the root disk
    DriveSerial,
}

impl UserHWIDComponent {
    fn to_machineid_enum(&self) -> HWIDComponent {
        use UserHWIDComponent::*;
        return match self {
            SystemID => HWIDComponent::SystemID,
            CPUCores => HWIDComponent::CPUCores,
            OSName => HWIDComponent::OSName,
            Username => HWIDComponent::Username,
            MachineName => HWIDComponent::MachineName,
            MacAddress => HWIDComponent::MacAddress,
            CPUID => HWIDComponent::CPUID,
            DriveSerial => HWIDComponent::DriveSerial,
        };
    }
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Cli::parse();

    let parts = if args.everything {
        UserHWIDComponent::iter().collect()
    } else {
        args.parts
    };

    let components: Vec<HWIDComponent> = parts
        .into_iter()
        .unique()
        .map(|part| part.to_machineid_enum())
        .collect();

    let mut builder = IdBuilder::new(Encryption::MD5);
    for component in components {
        builder.add_component(component);
    }

    /*
    if let Some(file) = args.token_file {
        builder.add_component(HWIDComponent::FileToken(file.to_str().unwrap()));
    }
    */

    let result = builder.build(&args.key).unwrap();
    println!("{result}");
}
