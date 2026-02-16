use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use iso_currency::IntoEnumIterator;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("currencies.rs");
    let mut f = File::create(&dest_path).unwrap();

    // Generate for ALL ISO currencies
    for currency in iso_currency::Currency::iter() {
        let code = currency.code();
        let symbol = currency.symbol();
        let name = currency.name();
        let exponent = currency.exponent().unwrap_or(0);

        // Write struct and impl for each currency
        writeln!(f, "
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct {};

impl Currency for {} {{
    const CODE: &'static str = \"{}\";
    const SYMBOL: &'static str = \"{}\";
    const NAME: &'static str = \"{}\";
    const DECIMAL_PLACES: u32 = {};
    const MINOR_UNIT_NAME: &'static str = \"\";
}}
", code, code, code, symbol, name, exponent).unwrap();
    }

    println!("cargo:rerun-if-changed=build.rs");
}
