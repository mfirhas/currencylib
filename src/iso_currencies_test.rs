use crate::*;

#[test]
fn test_derives() {
    assert_eq!(USD, USD);
    assert_eq!(EUR, EUR);
    assert_eq!(JPY, JPY);

    // //no need to assert these since it'a already statically checked at compile-time.
    // assert_ne!(USD, EUR);
    // assert_ne!(JPY, USD);
}

#[test]
fn test_usd_currency() {
    assert_eq!(USD::CODE, "USD");
    assert_eq!(USD::SYMBOL, "$");
    assert_eq!(USD::NAME, "United States dollar");
    assert_eq!(USD::NUMERIC, 840);
    assert_eq!(USD::MINOR_UNIT, 2);
    assert_eq!(USD::MINOR_UNIT_SYMBOL, "¢");
    assert_eq!(USD::THOUSAND_SEPARATOR, ",");
    assert_eq!(USD::DECIMAL_SEPARATOR, ".");
}
