#![forbid(clippy::unwrap_used)]

use iso_currency::IntoEnumIterator;
use phf::phf_set;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::LazyLock;

/// If a currency has no symbol gives it default.
const DEFAULT_MINOR_UNIT_SYMBOL: &str = "minor";

/// Where build.rs will put build result.
const OUT_FILENAME: &str = "iso_currencies.rs";

fn main() {
    generate_iso().expect("failed generating iso currencies");
}

/// Generate `Currency` implementations for all ISO 4217 currencies
fn generate_iso() -> Result<(), String> {
    let out_dir = env::var("OUT_DIR").map_err(|err| err.to_string())?;
    let dest_path = Path::new(&out_dir).join(OUT_FILENAME);
    let mut f = File::create(&dest_path).map_err(|err| err.to_string())?;

    writeln!(f, "use crate::Currency;").map_err(|err| err.to_string())?;

    // Generate for ALL ISO currencies
    for currency in iso_currency::Currency::iter() {
        let isocurrency: IsoCurrency = currency.into();

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
    const THOUSAND_SEPARATOR: &'static str = \"{}\";
    const DECIMAL_SEPARATOR: &'static str = \"{}\";
}}
",
            isocurrency.name.clone(),
            isocurrency.code,
            isocurrency.code,
            isocurrency.code,
            isocurrency.symbol,
            isocurrency.name,
            isocurrency.numeric,
            isocurrency.minor_unit,
            isocurrency.minor_unit_symbol,
            isocurrency.separator.thousand_separator,
            isocurrency.separator.decimal_separator,
        )
        .map_err(|err| err.to_string())?;
    }

    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}

/// List of countries whose currency's thousands separated by comma(",")
static COMMA_SEPARATED_THOUSAND_CURRENCIES: phf::Set<&'static str> = phf_set! {
        // North America
        "USD", // United States Dollar
        "CAD", // Canadian Dollar
        "MXN", // Mexican Peso
        // Central America & Caribbean
        "GTQ", // Guatemalan Quetzal
        "HNL", // Honduran Lempira
        "NIO", // Nicaraguan Córdoba
        "CRC", // Costa Rican Colón
        "PAB", // Panamanian Balboa
        "DOP", // Dominican Peso
        // Asia
        "CNY", // Chinese Yuan
        "JPY", // Japanese Yen
        "KRW", // South Korean Won
        "TWD", // Taiwan Dollar
        "THB", // Thai Baht
        "MYR", // Malaysian Ringgit
        "SGD", // Singapore Dollar
        "PHP", // Philippine Peso
        "INR", // Indian Rupee
        "PKR", // Pakistani Rupee
        "BDT", // Bangladeshi Taka
        "HKD", // Hong Kong Dollar
        "LKR", // Sri Lankan Rupee
        "NPR", // Nepalese Rupee
        // Africa
        "ZAR", // South African Rand
        "BWP", // Botswana Pula
        "ZMW", // Zambian Kwacha
        "KES", // Kenyan Shilling
        "TZS", // Tanzanian Shilling
        "UGX", // Ugandan Shilling
        "GHS", // Ghanaian Cedi
        "NGN", // Nigerian Naira
        // Oceania
        "AUD", // Australian Dollar
        "NZD", // New Zealand Dollar
        // Europe (isolated)
        "GBP", // British Pound
        "CHF", // Swiss Franc
        "ISK", // Icelandic Króna
        // Middle East
        "SAR", // Saudi Riyal
        "AED", // UAE Dirham
        //
        "ILS"
};

/// Facade for iso currencies
struct IsoCurrency {
    pub code: String,
    pub symbol: String,
    pub name: String,
    pub numeric: u16,
    pub minor_unit: u16,
    pub minor_unit_symbol: String,
    pub separator: Separator,
}

impl IsoCurrency {
    /// Override iso currency properties
    pub(crate) fn r#override<F>(&mut self, func: F)
    where
        F: FnOnce(&mut Self),
    {
        func(self)
    }
}

impl From<iso_currency::Currency> for IsoCurrency {
    fn from(currency: iso_currency::Currency) -> Self {
        let code = currency.code();
        let symbol = currency.symbol();
        let name = currency.name();
        let numeric = currency.numeric();
        let minor_unit = currency.exponent().unwrap_or_default();
        let minor_unit_symbol = if let Some(ref minor_symbol) = symbol.subunit_symbol {
            minor_symbol
        } else if minor_unit == 0 {
            ""
        } else {
            DEFAULT_MINOR_UNIT_SYMBOL
        };

        let separator: Separator = code.to_string().into();

        let mut isocurrency = Self {
            code: code.into(),
            symbol: symbol.to_string(),
            name: name.into(),
            numeric,
            minor_unit,
            minor_unit_symbol: minor_unit_symbol.into(),
            separator,
        };

        // overrides
        for func in OVERRIDES.iter() {
            isocurrency.r#override(func);
        }

        isocurrency
    }
}

struct Separator {
    pub thousand_separator: String,
    pub decimal_separator: String,
}

impl From<String> for Separator {
    fn from(value: String) -> Self {
        if let Some(c) = iso_currency::Currency::from_code(&value)
            && COMMA_SEPARATED_THOUSAND_CURRENCIES.contains(c.code())
        {
            Separator {
                thousand_separator: ",".into(),
                decimal_separator: ".".into(),
            }
        } else {
            Separator {
                thousand_separator: ".".into(),
                decimal_separator: ",".into(),
            }
        }
    }
}

/// List of iso currencies override functions.
/// Each overrides should have comment on why it overrides.
static OVERRIDES: LazyLock<Vec<fn(&mut IsoCurrency)>> = LazyLock::new(|| vec![]);
