mod cli;
mod config;
mod process;

use crate::cli::Cli;
use crate::config::{Config, RateType, DEFAULT_CONFIG};
use crate::process::{Cal, Interest, Principal, Repays};
use clap::Parser;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

fn main() {
    let cli: Cli = Cli::parse();

    if cli.generate {
        let mut file = File::create("./user.toml").unwrap();
        file.write_all(DEFAULT_CONFIG.as_bytes())
            .expect("write default config failed");
        return;
    }

    let config_path = if cli.config.is_none() {
        PathBuf::from("./user.toml")
    } else {
        cli.config.unwrap()
    };

    if let Ok(mut file) = File::open(config_path) {
        let mut buf = vec![];
        let _ = file.read_to_end(&mut buf);
        let config: Config = toml::from_slice(buf.as_slice()).unwrap();
        if cli.template {
            println!("Generate the user policy template for the config time(month)");
            let mut file = File::create("./policy.csv").unwrap();
            for i in 1..=config.loan.time {
                file.write_all(format!("{}, 0\n", i).as_bytes())
                    .expect("error");
            }
        } else {
            let policy = Repays::new("./policy.csv").unwrap();
            match config.rate.ty {
                RateType::Interest => process::mortgage_process(Interest {
                    config: &config,
                    policy: &policy,
                }),
                RateType::Principal => process::mortgage_process(Principal {
                    config: &config,
                    policy: &policy,
                }),
            };
        }
    } else {
        println!("Please use --gen to generate the default config file and retry.");
    }
}
