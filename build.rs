#![forbid(clippy::unwrap_used)]

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[path = "src/data.rs"]
mod data;

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

    // Collect sorted codes for deterministic output
    let mut codes: Vec<&&str> = data::ISO_CURRENCY_DATA.keys().collect();
    codes.sort();

    // Generate for ALL ISO currencies
    for code in &codes {
        let data::Data {
            code,
            symbol,
            name,
            numeric,
            minor_unit,
            minor_unit_symbol,
            minor_unit_name,
            thousand_separator,
            decimal_separator,
            origin,
            locale,
        } = data::ISO_CURRENCY_DATA
            .get(code)
            .ok_or(format!("currency code {} not found", code))?;

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
            name,
            code,
            code,
            code,
            symbol,
            name,
            numeric,
            minor_unit,
            minor_unit_symbol,
            minor_unit_name,
            thousand_separator,
            decimal_separator,
            origin,
            locale,
            code,
            code,
            code
        )
        .map_err(|err| err.to_string())?;
    }

    // Generate the IsoCurrency enum with a variant per currency
    writeln!(
        f,
        "
/// A dynamic enum representing all ISO 4217 currency marker types.
///
/// Use [`IsoCurrency::from_str`] (or `.parse::<IsoCurrency>()`) to obtain a value from a
/// currency code string such as `\"USD\"` or `\"EUR\"`. Each variant corresponds to the
/// zero-sized marker struct of the same name.
///
/// # Example
/// ```
/// use core::str::FromStr;
/// use currencylib::IsoCurrency;
///
/// let c = IsoCurrency::from_str(\"USD\").unwrap();
/// assert_eq!(c, IsoCurrency::USD);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IsoCurrency {{"
    )
    .map_err(|err| err.to_string())?;

    for code in &codes {
        let name = data::ISO_CURRENCY_DATA
            .get(code)
            .ok_or(format!("currency code {} not found", code))?
            .name;
        writeln!(f, "    /// {}", name).map_err(|err| err.to_string())?;
        writeln!(f, "    {},", code).map_err(|err| err.to_string())?;
    }

    writeln!(f, "}}").map_err(|err| err.to_string())?;

    // Generate FromStr for IsoCurrency
    writeln!(
        f,
        "
impl FromStr for IsoCurrency {{
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {{
        let s = s.trim();
        match s {{"
    )
    .map_err(|err| err.to_string())?;

    for code in &codes {
        writeln!(f, "            \"{}\" => Ok(IsoCurrency::{}),", code, code)
            .map_err(|err| err.to_string())?;
    }

    writeln!(
        f,
        "            _ => Err(\"unknown currency code\"),
        }}
    }}
}}"
    )
    .map_err(|err| err.to_string())?;

    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
