use crate::Config;
use pager::Pager;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use std::collections::HashMap;
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
pub struct Repays(HashMap<u64, Decimal>);

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
                        let month = res[0].parse().unwrap();
                        let repay = Decimal::from_str(res[1]).unwrap() * dec!(10000);
                        if month < 12 && repay > Decimal::ZERO {
                            panic!("forbid first year repay");
                        }
                        (month, repay)
                    } else {
                        panic!("readline error policy csv");
                    }
                })
                .collect(),
        ))
    }
}

pub struct Principal<'a> {
    pub config: &'a Config,
    pub policy: &'a Repays,
}

// https://www.cnblogs.com/lhws/archive/2013/04/12/3017246.html
impl<'a> Cal for Principal<'a> {
    fn process(&self) {
        let time = self.config.loan.time as u64;
        println!("等额本金 总共{}月", time);
        let fund = self.config.loan.fund * dec!(10000);
        let fund_rate = self.config.rate.fund / dec!(100) / dec!(12);
        let business = self.config.loan.business * dec!(10000);
        let business_rate = self.config.rate.business / dec!(100) / dec!(12);

        let fund_every_month = fund / Decimal::from(time);
        let business_every_month = business / Decimal::from(time);

        println!(
            "商贷总额:{:.2} 利率:{:.4} 总利息:{:.2} \n公积金总额:{:.2} 利率:{:.4} 总利息:{:.2}\n 总利息:{:.2}",
            business,
            business_rate,
            Decimal::from(time + 1) * business * business_rate / dec!(2),
            fund,
            fund_rate,
            Decimal::from(time + 1) * fund * fund_rate / dec!(2),
            Decimal::from(time + 1) * business * business_rate / dec!(2)
                + Decimal::from(time + 1) * fund * fund_rate / dec!(2)
        );
        let mut total_in = Decimal::ZERO;

        let mut already_repay_f_p = Decimal::ZERO;
        let mut already_repay_b_p = Decimal::ZERO;

        for i in 0..time {
            let f_repay = fund_every_month + (fund - already_repay_f_p) * fund_rate;
            let f_repay_i = (fund - already_repay_f_p) * fund_rate;

            let b_repay = business_every_month + (business - already_repay_b_p) * business_rate;
            let b_repay_i = (business - already_repay_b_p) * business_rate;

            total_in = total_in + f_repay_i + b_repay_i;

            already_repay_f_p += fund_every_month;
            already_repay_b_p = already_repay_b_p
                + business_every_month
                + if let Some(repay) = self.policy.0.get(&(i + 1)) {
                    repay
                } else {
                    println!("warning {} month repay parse failed", i + 1);
                    &Decimal::ZERO
                };
            println!(
                "{}月\n公积金 本金{:.2} 利息{:.2} 总计:{:.2}\n商贷 本金{:.2} 利息{:.2} 总计:{:.2}\n总计 本金{:.2} 利息{:.2} 总计:{:.2}",
                i + 1,
                fund_every_month,
                f_repay_i, f_repay, business_every_month, b_repay_i, b_repay, fund_every_month + business_every_month, f_repay_i + b_repay_i, f_repay + b_repay,
            );
        }
        println!("total interest {:.2}", total_in);
    }
}

pub struct Interest<'a> {
    pub config: &'a Config,
    pub policy: &'a Repays,
}

fn interest_cal(number: Decimal, rate: Decimal, time: u64) -> Decimal {
    number * (rate * (rate + dec!(1)).powu(time)) / ((rate + dec!(1)).powu(time) - dec!(1))
}

// https://zhuanlan.zhihu.com/p/390581715
impl<'a> Cal for Interest<'a> {
    fn process(&self) {
        let time = self.config.loan.time as u64;
        println!("等额本息 总共{}月", time);

        let fund = self.config.loan.fund * dec!(10000);
        let fund_rate = self.config.rate.fund / dec!(100) / dec!(12);
        let fund_month = interest_cal(fund, fund_rate, time);

        let business = self.config.loan.business * dec!(10000);
        let business_rate = self.config.rate.business / dec!(100) / dec!(12);
        let business_month = interest_cal(business, business_rate, time);

        let total_month = fund_month + business_month;
        println!(
            "商贷总额:{:.2} 利率:{:.4} 总利息:{:.2} \n公积金总额:{:.2} 利率:{:.4} 总利息:{:.2}\n 总利息:{:.2}",
            business,
            business_rate,
            business_month * Decimal::from(time) - business,
            fund,
            fund_rate,
            fund_month * Decimal::from(time) - fund,
            business_month * Decimal::from(time) - business + fund_month * Decimal::from(time)
                - fund
        );

        println!(
            "every month fund:{:.2} business:{:.2} total:{:.2}",
            fund_month, business_month, total_month,
        );

        let mut total_interest = Decimal::ZERO;

        // let business_first_repay_principal = business_month - business * business_rate;
        // let fund_first_repay_principal = fund_month - fund * fund_rate;

        for i in 0..time {
            // https://baike.baidu.com/item/%E7%AD%89%E9%A2%9D%E6%9C%AC%E6%81%AF%E6%B3%95/11049926
            // https://www.cnblogs.com/hanganglin/p/6777838.html
            let f_i = (fund * fund_rate - fund_month) * (dec!(1) + fund_rate).powu(i) + fund_month;
            // let f_p = fund_first_repay_principal * (dec!(1) + fund_rate).powu(i);
            let f_p = fund_month - f_i;

            // let b_p = business_first_repay_principal * (dec!(1) + business_rate).powu(i);
            let b_i = (business * business_rate - business_month)
                * (dec!(1) + business_rate).powu(i)
                + business_month;
            let b_p = business_month - b_i;

            total_interest = total_interest + f_i + b_i;
            println!(
                "{}月\n公积金 本金{:.2} 利息{:.2} 总计:{:.2}\n商贷 本金{:.2} 利息{:.2} 总计:{:.2}\n总计 本金{:.2} 利息{:.2} 总计:{:.2}",
                i + 1,
                f_p,
                f_i, f_p + f_i, b_p, b_i, b_p + b_i, f_p + b_p, f_i + b_i, f_p + b_p + f_i + b_i
            );
        }
        println!("total interest {:.2}", total_interest);
    }
}
