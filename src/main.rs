extern crate core;

use crate::base_converter::convert_dec_to_hex;
use crate::utils::ConversionError;

mod base_converter;
mod utils;

fn main() {
    let num_str = "-115".to_string();
     match convert_dec_to_hex(&num_str)
    {
        Ok(v) =>
            {
                let hex = format!("0x{}", v);
                println!("{}", hex);
            }
        Err(e) =>
            {
                println!("{}", e);
                return;
            },
    };


}
