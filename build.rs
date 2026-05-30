#![forbid(clippy::unwrap_used)]

use ::data::currencies;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Where build.rs will put build result.
const ISO_OUT_FILENAME: &str = "iso_currencies.rs";

fn main() {
    generate_iso().unwrap_or_else(|err| panic!("failed generating iso currencies: {}", err));
}

/// Generate `Currency` implementations for all ISO 4217 currencies
fn generate_iso() -> Result<(), String> {
    let out_dir = env::var("OUT_DIR").map_err(|err| err.to_string())?;
    let dest_path = Path::new(&out_dir).join(ISO_OUT_FILENAME);
    let mut f = File::create(&dest_path).map_err(|err| err.to_string())?;

    writeln!(f, "use crate::Currency;").map_err(|err| err.to_string())?;
    writeln!(f, "use core::str::FromStr;").map_err(|err| err.to_string())?;

    // Generate for ALL ISO currencies
    for (_, data) in currencies::entries() {
        // Write struct and impl for each currency
        writeln!(
            f,
            "
/// {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct {};

impl Currency for {} {{
    const CODE: &'static str = \"{}\";
    const SYMBOL: &'static str = \"{}\";
    const NAME: &'static str = \"{}\";
    const NUMERIC: u16 = {};
    const MINOR_UNIT: u16 = {};
    const MINOR_UNIT_SYMBOL: &'static str = \"{}\";
    const MINOR_UNIT_NAME: &'static str = \"{}\";
    const THOUSAND_SEPARATOR: &'static str = \"{}\";
    const DECIMAL_SEPARATOR: &'static str = \"{}\";
    const ORIGIN: &'static str = \"{}\";
    const LOCALE: &'static str = \"{}\";
}}

impl FromStr for {} {{
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {{
        let s = s.trim();
        if s != {}::CODE {{
            return Err(\"invalid currency code\");
        }}

        Ok({})
    }}
}}
",
            data.name,
            data.code,
            data.code,
            data.code,
            data.symbol,
            data.name,
            data.numeric,
            data.minor_unit,
            data.minor_unit_symbol,
            data.minor_unit_name,
            data.thousand_separator,
            data.decimal_separator,
            data.origin,
            data.locale,
            data.code,
            data.code,
            data.code
        )
        .map_err(|err| err.to_string())?;
    }

    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
