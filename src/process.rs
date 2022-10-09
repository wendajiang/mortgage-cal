use crate::Config;
use pager::Pager;
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
    Pager::with_pager("less -SR").setup();
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

// https://www.cnblogs.com/lhws/archive/2013/04/12/3017246.html
impl<'a> Cal for Principal<'a> {
    fn process(&self) {
        println!("等额本金");
        let time = self.config.loan.time as u64;
        let fund = self.config.loan.fund * dec!(10000);
        let fund_rate = self.config.rate.fund / dec!(100) / dec!(12);
        let business = self.config.loan.business * dec!(10000);
        let business_rate = self.config.rate.business / dec!(100) / dec!(12);

        let fund_every_month = fund / Decimal::from(time);
        let business_every_month = business / Decimal::from(time);

        for i in 0..time {
            let already_repay_f_p = fund_every_month * Decimal::from(i);
            let f_repay = fund_every_month + (fund - already_repay_f_p) * fund_rate;
            let f_repay_i = (fund - already_repay_f_p) * fund_rate;

            let already_repay_b_p = business_every_month * Decimal::from(i);
            let b_repay = business_every_month + (business - already_repay_b_p) * business_rate;
            let b_repay_i = (business - already_repay_b_p) * business_rate;
            println!(
                "{}月\n公积金 本金{:.2} 利息{:.2} 总计:{:.2}\n商贷 本金{:.2} 利息{:.2} 总计:{:.2}\n总计 本金{:.2} 利息{:.2} 总计:{:.2}",
                i + 1,
                fund_every_month,
                f_repay_i, f_repay, business_every_month, b_repay_i, b_repay, fund_every_month + business_every_month, f_repay_i + b_repay_i, f_repay + b_repay,
            );
        }
    }
}

pub struct Interest<'a> {
    pub config: &'a Config,
}

fn interest_cal(number: Decimal, rate: Decimal, time: u64) -> Decimal {
    number * (rate * (rate + dec!(1)).powu(time)) / ((rate + dec!(1)).powu(time) - dec!(1))
}

// https://zhuanlan.zhihu.com/p/390581715
impl<'a> Cal for Interest<'a> {
    fn process(&self) {
        println!("等额本息");
        let time = self.config.loan.time as u64;

        let fund = self.config.loan.fund * dec!(10000);
        let fund_rate = self.config.rate.fund / dec!(100) / dec!(12);
        let fund_month = interest_cal(fund, fund_rate, time);

        let business = self.config.loan.business * dec!(10000);
        let business_rate = self.config.rate.business / dec!(100) / dec!(12);
        let business_month = interest_cal(business, business_rate, time);
        let total_month = fund_month + business_month;
        println!(
            "every month fund:{:.2} business:{:.2} total:{:.2}",
            fund_month, business_month, total_month,
        );

        let mut total_interest = Decimal::ZERO;

        for i in 0..time {
            let remain_fund = fund - fund_month * Decimal::from(i);
            let f_i = remain_fund * fund_rate;
            let f_p = fund_month - f_i;

            let remain_business = business - business_month * Decimal::from(i);
            let b_i = remain_business * business_rate;
            let b_p = business_month - b_i;

            total_interest += f_i + b_i;
            println!(
                "{}月\n公积金 本金{:.2} 利息{:.2}\n商贷 本金{:.2} 利息{:.2}\n总计 本金{:.2} 利息{:.2}",
                i + 1,
                f_p,
                f_i, b_p, b_i, f_p + b_p, f_i + b_i,
            );
        }

        println!("total interest: {}", total_interest);
    }
}
