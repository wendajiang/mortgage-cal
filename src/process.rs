use crate::Config;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub trait Cal {
    fn process(&self);
}

pub fn mortgage_process<T>(ins: T)
where
    T: Cal,
{
    ins.process();
}

#[derive(Debug)]
pub struct Repays(Vec<Repay>);

#[derive(Debug)]
pub struct Repay {
    month: u32,
    repay: Decimal,
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

impl Repays {
    pub fn new<P>(path: P) -> anyhow::Result<Repays>
    where
        P: AsRef<Path>,
    {
        let lines = read_lines(path)?;
        Ok(Repays(
            lines
                .into_iter()
                .map(|line| {
                    if let Ok(line) = line {
                        let res: Vec<&str> = line
                            .split(',')
                            .into_iter()
                            .map(|x| x.trim_start().trim_end())
                            .collect();
                        Repay {
                            month: res[0].parse().unwrap(),
                            repay: Decimal::from_str(res[1]).unwrap(),
                        }
                    } else {
                        Repay {
                            month: 0,
                            repay: dec!(0),
                        }
                    }
                })
                .collect(),
        ))
    }
}

pub struct Principal<'a> {
    pub config: &'a Config,
}

impl<'a> Cal for Principal<'a> {
    fn process(&self) {
        println!("PPP");
        todo!()
    }
}

pub struct Interest<'a> {
    pub config: &'a Config,
}

impl<'a> Cal for Interest<'a> {
    fn process(&self) {
        println!("III");
        todo!()
    }
}
