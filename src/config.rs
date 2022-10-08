use rust_decimal::prelude::*;
use serde_derive::{Deserialize, Serialize};

pub const DEFAULT_CONFIG: &str = r##"[loan]
# w
business = 100
fund = 50
# month
time = 240

[rate]
# percent
business = 4.3
fund = 3.1
# type is Principal or Interest 
type = "Interest"
"##;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub loan: Loan,
    pub rate: Rate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Loan {
    pub business: Decimal,
    pub fund: Decimal,
    pub time: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rate {
    pub business: Decimal,
    pub fund: Decimal,
    #[serde(rename = "type")]
    pub ty: RateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RateType {
    Interest,
    Principal,
}
