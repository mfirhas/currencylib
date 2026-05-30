use crate::currencies;

#[test]
fn test_get_known_currency() {
    let usd = currencies::get("USD").expect("USD should be found");
    assert_eq!(usd.code, "USD");
    assert_eq!(usd.symbol, "$");
    assert_eq!(usd.name, "United States dollar");
    assert_eq!(usd.numeric, 840);
    assert_eq!(usd.minor_unit, 2);
    assert_eq!(usd.minor_unit_symbol, "¢");
    assert_eq!(usd.minor_unit_name, "cent");
    assert_eq!(usd.thousand_separator, ",");
    assert_eq!(usd.decimal_separator, ".");
    assert_eq!(usd.origin, "United States");
    assert_eq!(usd.locale, "en-US");
}

#[test]
fn test_get_eur_currency() {
    let eur = currencies::get("EUR").expect("EUR should be found");
    assert_eq!(eur.code, "EUR");
    assert_eq!(eur.symbol, "€");
    assert_eq!(eur.name, "Euro");
    assert_eq!(eur.numeric, 978);
    assert_eq!(eur.minor_unit, 2);
    assert_eq!(eur.minor_unit_symbol, "c");
    assert_eq!(eur.minor_unit_name, "cent");
    assert_eq!(eur.origin, "Eurozone");
    assert_eq!(eur.locale, "de-DE");
}

#[test]
fn test_get_unknown_currency() {
    assert!(currencies::get("XYZ").is_none());
    assert!(currencies::get("").is_none());
    assert!(currencies::get("usd").is_none());
}

#[test]
fn test_get_returns_copy() {
    let d1 = currencies::get("JPY").expect("JPY should be found");
    let d2 = currencies::get("JPY").expect("JPY should be found");
    assert_eq!(d1.code, d2.code);
    assert_eq!(d1.numeric, d2.numeric);
}

#[test]
fn test_iso_currency_data_map() {
    let aed = currencies::ISO_CURRENCY_DATA
        .get("AED")
        .expect("AED should be in map");
    assert_eq!(aed.code, "AED");
    assert_eq!(aed.origin, "United Arab Emirates");
}
