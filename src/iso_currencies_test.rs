use super::*;

#[test]
fn test_usd_currency() {
    assert_eq!(USD::CODE, "USD");
    assert_eq!(USD::SYMBOL, "$");
    assert_eq!(USD::NAME, "United States dollar");
    assert_eq!(USD::MINOR_UNIT, 2);
    assert_eq!(USD::MINOR_UNIT_SYMBOL, "¢");
}

#[test]
fn test_eur_currency() {
    assert_eq!(EUR::CODE, "EUR");
    assert_eq!(EUR::SYMBOL, "€");
    assert_eq!(EUR::NAME, "Euro");
    assert_eq!(EUR::MINOR_UNIT, 2);
    assert_eq!(EUR::MINOR_UNIT_SYMBOL, "minor");
}

#[test]
fn test_jpy_currency() {
    assert_eq!(JPY::CODE, "JPY");
    assert_eq!(JPY::SYMBOL, "¥");
    assert_eq!(JPY::NAME, "Japanese yen");
    assert_eq!(JPY::MINOR_UNIT, 0);
    assert_eq!(JPY::MINOR_UNIT_SYMBOL, "");
}

#[test]
fn test_gbp_currency() {
    assert_eq!(GBP::CODE, "GBP");
    assert_eq!(GBP::SYMBOL, "£");
    assert_eq!(GBP::NAME, "Pound sterling");
    assert_eq!(GBP::MINOR_UNIT, 2);
}

#[test]
fn test_chf_currency() {
    assert_eq!(CHF::CODE, "CHF");
    assert_eq!(CHF::NAME, "Swiss franc");
    assert_eq!(CHF::MINOR_UNIT, 2);
}

#[test]
fn test_cad_currency() {
    assert_eq!(CAD::CODE, "CAD");
    assert_eq!(CAD::SYMBOL, "$");
    assert_eq!(CAD::NAME, "Canadian dollar");
}

#[test]
fn test_aud_currency() {
    assert_eq!(AUD::CODE, "AUD");
    assert_eq!(AUD::NAME, "Australian dollar");
}

#[test]
fn test_cny_currency() {
    assert_eq!(CNY::CODE, "CNY");
    assert_eq!(CNY::NAME, "Renminbi (Chinese) yuan");
}

#[test]
fn test_inr_currency() {
    assert_eq!(INR::CODE, "INR");
    assert_eq!(INR::NAME, "Indian rupee");
}

#[test]
fn test_brl_currency() {
    assert_eq!(BRL::CODE, "BRL");
    assert_eq!(BRL::NAME, "Brazilian real");
}
