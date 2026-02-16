use iso_currency::IntoEnumIterator;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    generate_iso().expect("failed generating currencies");
}

/// Generate `Currency` implementations for all ISO 4217 currencies
fn generate_iso() -> Result<(), String> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("currencies.rs");
    let mut f = File::create(&dest_path).unwrap();

    const DEFAULT_MINOR_UNIT_SYMBOL: &str = "minor";

    // Generate for ALL ISO currencies
    for currency in iso_currency::Currency::iter() {
        let code = currency.code();
        let symbol = currency.symbol();
        let name = currency.name();
        let numeric = currency.numeric();
        let exponent = currency.exponent().unwrap_or_default();
        let minor_unit_symbol = if let Some(ref minor_symbol) = symbol.subunit_symbol {
            minor_symbol
        } else if exponent == 0 {
            ""
        } else {
            DEFAULT_MINOR_UNIT_SYMBOL
        };

        // Write struct and impl for each currency
        writeln!(
            f,
            "
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct {};

impl Currency for {} {{
    const CODE: &'static str = \"{}\";
    const SYMBOL: &'static str = \"{}\";
    const NAME: &'static str = \"{}\";
    const NUMERIC: u16 = {};
    const MINOR_UNIT: u16 = {};
    const MINOR_UNIT_SYMBOL: &'static str = \"{}\";
}}
",
            code, code, code, symbol, name, numeric, exponent, minor_unit_symbol
        )
        .map_err(|err| err.to_string())?;
    }

    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
