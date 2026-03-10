#![forbid(clippy::unwrap_used)]

use iso_currency::IntoEnumIterator;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;
use std::sync::LazyLock;

#[path = "locale.rs"]
mod locale;

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
    writeln!(f, "use core::str::FromStr;").map_err(|err| err.to_string())?;

    // Generate for ALL ISO currencies
    for currency in iso_currency::Currency::iter() {
        let isocurrency: IsoCurrency = currency.try_into()?;

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
            isocurrency.name,
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
            isocurrency.code,
            isocurrency.code,
            isocurrency.code
        )
        .map_err(|err| err.to_string())?;
    }

    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}

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

impl TryFrom<iso_currency::Currency> for IsoCurrency {
    type Error = String;

    fn try_from(currency: iso_currency::Currency) -> Result<Self, Self::Error> {
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

        let separator = code.parse::<Separator>()?;

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

        Ok(isocurrency)
    }
}

struct Separator {
    pub thousand_separator: String,
    pub decimal_separator: String,
}

impl FromStr for Separator {
    type Err = String;

    /// Instantiate Separator from currency alpha code.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use icu::decimal::provider::{Baked, DecimalSymbolsV1};
        use icu::locale::Locale;
        use icu_provider::prelude::*;

        if let Some(loc_code) = locale::CURRENCY_TO_LOCALE.get(s) {
            let loc = loc_code.parse::<Locale>().map_err(|err| err.to_string())?;

            let data_locale: DataLocale = (&loc).into();
            let id = DataIdentifierBorrowed::for_locale(&data_locale);

            let resp: DataResponse<DecimalSymbolsV1> = Baked
                .load(DataRequest {
                    id,
                    ..Default::default()
                })
                .map_err(|err| err.to_string())?;

            let symbols = resp.payload.get();

            Ok(Self {
                thousand_separator: symbols.grouping_separator().into(),
                decimal_separator: symbols.decimal_separator().into(),
            })
        } else {
            Err(format!("Currency Code {} not found in ISO 4217", s))
        }
    }
}

/// List of iso currencies override functions.
/// Each overrides should have comment on why it overrides.
static OVERRIDES: LazyLock<Vec<fn(&mut IsoCurrency)>> = LazyLock::new(|| vec![]);
