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
fn test_aed_currency() {
    assert_eq!(AED::CODE, "AED");
    assert_eq!(AED::SYMBOL, "د.إ");
    assert_eq!(AED::NAME, "United Arab Emirates dirham");
    assert_eq!(AED::NUMERIC, 784);
    assert_eq!(AED::MINOR_UNIT, 2);
    assert_eq!(AED::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(AED::THOUSAND_SEPARATOR, ",");
    assert_eq!(AED::DECIMAL_SEPARATOR, ".");
}

#[test]
fn test_afn_currency() {
    assert_eq!(AFN::CODE, "AFN");
    assert_eq!(AFN::SYMBOL, "؋");
    assert_eq!(AFN::NAME, "Afghan afghani");
    assert_eq!(AFN::NUMERIC, 971);
    assert_eq!(AFN::MINOR_UNIT, 2);
    assert_eq!(AFN::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(AFN::THOUSAND_SEPARATOR, ".");
    assert_eq!(AFN::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_all_currency() {
    assert_eq!(ALL::CODE, "ALL");
    assert_eq!(ALL::SYMBOL, "L");
    assert_eq!(ALL::NAME, "Albanian lek");
    assert_eq!(ALL::NUMERIC, 8);
    assert_eq!(ALL::MINOR_UNIT, 2);
    assert_eq!(ALL::MINOR_UNIT_SYMBOL, "q");
    assert_eq!(ALL::THOUSAND_SEPARATOR, ".");
    assert_eq!(ALL::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_amd_currency() {
    assert_eq!(AMD::CODE, "AMD");
    assert_eq!(AMD::SYMBOL, "֏");
    assert_eq!(AMD::NAME, "Armenian dram");
    assert_eq!(AMD::NUMERIC, 51);
    assert_eq!(AMD::MINOR_UNIT, 2);
    assert_eq!(AMD::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(AMD::THOUSAND_SEPARATOR, ".");
    assert_eq!(AMD::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_aoa_currency() {
    assert_eq!(AOA::CODE, "AOA");
    assert_eq!(AOA::SYMBOL, "Kz");
    assert_eq!(AOA::NAME, "Angolan kwanza");
    assert_eq!(AOA::NUMERIC, 973);
    assert_eq!(AOA::MINOR_UNIT, 2);
    assert_eq!(AOA::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(AOA::THOUSAND_SEPARATOR, ".");
    assert_eq!(AOA::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_ars_currency() {
    assert_eq!(ARS::CODE, "ARS");
    assert_eq!(ARS::SYMBOL, "$");
    assert_eq!(ARS::NAME, "Argentine peso");
    assert_eq!(ARS::NUMERIC, 32);
    assert_eq!(ARS::MINOR_UNIT, 2);
    assert_eq!(ARS::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(ARS::THOUSAND_SEPARATOR, ".");
    assert_eq!(ARS::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_aud_currency() {
    assert_eq!(AUD::CODE, "AUD");
    assert_eq!(AUD::SYMBOL, "$");
    assert_eq!(AUD::NAME, "Australian dollar");
    assert_eq!(AUD::NUMERIC, 36);
    assert_eq!(AUD::MINOR_UNIT, 2);
    assert_eq!(AUD::MINOR_UNIT_SYMBOL, "c");
    assert_eq!(AUD::THOUSAND_SEPARATOR, ",");
    assert_eq!(AUD::DECIMAL_SEPARATOR, ".");
}

#[test]
fn test_awg_currency() {
    assert_eq!(AWG::CODE, "AWG");
    assert_eq!(AWG::SYMBOL, "ƒ");
    assert_eq!(AWG::NAME, "Aruban florin");
    assert_eq!(AWG::NUMERIC, 533);
    assert_eq!(AWG::MINOR_UNIT, 2);
    assert_eq!(AWG::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(AWG::THOUSAND_SEPARATOR, ".");
    assert_eq!(AWG::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_azn_currency() {
    assert_eq!(AZN::CODE, "AZN");
    assert_eq!(AZN::SYMBOL, "₼");
    assert_eq!(AZN::NAME, "Azerbaijani manat");
    assert_eq!(AZN::NUMERIC, 944);
    assert_eq!(AZN::MINOR_UNIT, 2);
    assert_eq!(AZN::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(AZN::THOUSAND_SEPARATOR, ".");
    assert_eq!(AZN::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_bam_currency() {
    assert_eq!(BAM::CODE, "BAM");
    assert_eq!(BAM::SYMBOL, "KM");
    assert_eq!(BAM::NAME, "Bosnia and Herzegovina convertible mark");
    assert_eq!(BAM::NUMERIC, 977);
    assert_eq!(BAM::MINOR_UNIT, 2);
    assert_eq!(BAM::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(BAM::THOUSAND_SEPARATOR, ".");
    assert_eq!(BAM::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_bbd_currency() {
    assert_eq!(BBD::CODE, "BBD");
    assert_eq!(BBD::SYMBOL, "Bds$");
    assert_eq!(BBD::NAME, "Barbados dollar");
    assert_eq!(BBD::NUMERIC, 52);
    assert_eq!(BBD::MINOR_UNIT, 2);
    assert_eq!(BBD::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(BBD::THOUSAND_SEPARATOR, ".");
    assert_eq!(BBD::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_bdt_currency() {
    assert_eq!(BDT::CODE, "BDT");
    assert_eq!(BDT::SYMBOL, "৳");
    assert_eq!(BDT::NAME, "Bangladeshi taka");
    assert_eq!(BDT::NUMERIC, 50);
    assert_eq!(BDT::MINOR_UNIT, 2);
    assert_eq!(BDT::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(BDT::THOUSAND_SEPARATOR, ",");
    assert_eq!(BDT::DECIMAL_SEPARATOR, ".");
}

#[test]
fn test_bgn_currency() {
    assert_eq!(BGN::CODE, "BGN");
    assert_eq!(BGN::SYMBOL, "лв.");
    assert_eq!(BGN::NAME, "Bulgarian lev");
    assert_eq!(BGN::NUMERIC, 975);
    assert_eq!(BGN::MINOR_UNIT, 2);
    assert_eq!(BGN::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(BGN::THOUSAND_SEPARATOR, ".");
    assert_eq!(BGN::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_bhd_currency() {
    assert_eq!(BHD::CODE, "BHD");
    assert_eq!(BHD::SYMBOL, ".د.ب");
    assert_eq!(BHD::NAME, "Bahraini dinar");
    assert_eq!(BHD::NUMERIC, 48);
    assert_eq!(BHD::MINOR_UNIT, 3);
    assert_eq!(BHD::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(BHD::THOUSAND_SEPARATOR, ".");
    assert_eq!(BHD::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_bif_currency() {
    assert_eq!(BIF::CODE, "BIF");
    assert_eq!(BIF::SYMBOL, "FBu");
    assert_eq!(BIF::NAME, "Burundian franc");
    assert_eq!(BIF::NUMERIC, 108);
    assert_eq!(BIF::MINOR_UNIT, 0);
    assert_eq!(BIF::MINOR_UNIT_SYMBOL, "");
    assert_eq!(BIF::THOUSAND_SEPARATOR, ".");
    assert_eq!(BIF::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_bmd_currency() {
    assert_eq!(BMD::CODE, "BMD");
    assert_eq!(BMD::SYMBOL, "$");
    assert_eq!(BMD::NAME, "Bermudian dollar");
    assert_eq!(BMD::NUMERIC, 60);
    assert_eq!(BMD::MINOR_UNIT, 2);
    assert_eq!(BMD::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(BMD::THOUSAND_SEPARATOR, ".");
    assert_eq!(BMD::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_bnd_currency() {
    assert_eq!(BND::CODE, "BND");
    assert_eq!(BND::SYMBOL, "B$");
    assert_eq!(BND::NAME, "Brunei dollar");
    assert_eq!(BND::NUMERIC, 96);
    assert_eq!(BND::MINOR_UNIT, 2);
    assert_eq!(BND::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(BND::THOUSAND_SEPARATOR, ".");
    assert_eq!(BND::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_bob_currency() {
    assert_eq!(BOB::CODE, "BOB");
    assert_eq!(BOB::SYMBOL, "Bs.");
    assert_eq!(BOB::NAME, "Boliviano");
    assert_eq!(BOB::NUMERIC, 68);
    assert_eq!(BOB::MINOR_UNIT, 2);
    assert_eq!(BOB::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(BOB::THOUSAND_SEPARATOR, ".");
    assert_eq!(BOB::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_bov_currency() {
    assert_eq!(BOV::CODE, "BOV");
    assert_eq!(BOV::SYMBOL, "¤");
    assert_eq!(BOV::NAME, "Bolivian Mvdol");
    assert_eq!(BOV::NUMERIC, 984);
    assert_eq!(BOV::MINOR_UNIT, 2);
    assert_eq!(BOV::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(BOV::THOUSAND_SEPARATOR, ".");
    assert_eq!(BOV::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_brl_currency() {
    assert_eq!(BRL::CODE, "BRL");
    assert_eq!(BRL::SYMBOL, "R$");
    assert_eq!(BRL::NAME, "Brazilian real");
    assert_eq!(BRL::NUMERIC, 986);
    assert_eq!(BRL::MINOR_UNIT, 2);
    assert_eq!(BRL::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(BRL::THOUSAND_SEPARATOR, ".");
    assert_eq!(BRL::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_bsd_currency() {
    assert_eq!(BSD::CODE, "BSD");
    assert_eq!(BSD::SYMBOL, "$");
    assert_eq!(BSD::NAME, "Bahamian dollar");
    assert_eq!(BSD::NUMERIC, 44);
    assert_eq!(BSD::MINOR_UNIT, 2);
    assert_eq!(BSD::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(BSD::THOUSAND_SEPARATOR, ".");
    assert_eq!(BSD::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_btn_currency() {
    assert_eq!(BTN::CODE, "BTN");
    assert_eq!(BTN::SYMBOL, "Nu.");
    assert_eq!(BTN::NAME, "Bhutanese ngultrum");
    assert_eq!(BTN::NUMERIC, 64);
    assert_eq!(BTN::MINOR_UNIT, 2);
    assert_eq!(BTN::MINOR_UNIT_SYMBOL, "Ch.");
    assert_eq!(BTN::THOUSAND_SEPARATOR, ".");
    assert_eq!(BTN::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_bwp_currency() {
    assert_eq!(BWP::CODE, "BWP");
    assert_eq!(BWP::SYMBOL, "P");
    assert_eq!(BWP::NAME, "Botswana pula");
    assert_eq!(BWP::NUMERIC, 72);
    assert_eq!(BWP::MINOR_UNIT, 2);
    assert_eq!(BWP::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(BWP::THOUSAND_SEPARATOR, ",");
    assert_eq!(BWP::DECIMAL_SEPARATOR, ".");
}

#[test]
fn test_byn_currency() {
    assert_eq!(BYN::CODE, "BYN");
    assert_eq!(BYN::SYMBOL, "Br");
    assert_eq!(BYN::NAME, "Belarusian ruble");
    assert_eq!(BYN::NUMERIC, 933);
    assert_eq!(BYN::MINOR_UNIT, 2);
    assert_eq!(BYN::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(BYN::THOUSAND_SEPARATOR, ".");
    assert_eq!(BYN::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_bzd_currency() {
    assert_eq!(BZD::CODE, "BZD");
    assert_eq!(BZD::SYMBOL, "$");
    assert_eq!(BZD::NAME, "Belize dollar");
    assert_eq!(BZD::NUMERIC, 84);
    assert_eq!(BZD::MINOR_UNIT, 2);
    assert_eq!(BZD::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(BZD::THOUSAND_SEPARATOR, ".");
    assert_eq!(BZD::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_cad_currency() {
    assert_eq!(CAD::CODE, "CAD");
    assert_eq!(CAD::SYMBOL, "$");
    assert_eq!(CAD::NAME, "Canadian dollar");
    assert_eq!(CAD::NUMERIC, 124);
    assert_eq!(CAD::MINOR_UNIT, 2);
    assert_eq!(CAD::MINOR_UNIT_SYMBOL, "¢");
    assert_eq!(CAD::THOUSAND_SEPARATOR, ",");
    assert_eq!(CAD::DECIMAL_SEPARATOR, ".");
}

#[test]
fn test_cdf_currency() {
    assert_eq!(CDF::CODE, "CDF");
    assert_eq!(CDF::SYMBOL, "₣");
    assert_eq!(CDF::NAME, "Congolese franc");
    assert_eq!(CDF::NUMERIC, 976);
    assert_eq!(CDF::MINOR_UNIT, 2);
    assert_eq!(CDF::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(CDF::THOUSAND_SEPARATOR, ".");
    assert_eq!(CDF::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_che_currency() {
    assert_eq!(CHE::CODE, "CHE");
    assert_eq!(CHE::SYMBOL, "¤");
    assert_eq!(CHE::NAME, "WIR Euro");
    assert_eq!(CHE::NUMERIC, 947);
    assert_eq!(CHE::MINOR_UNIT, 2);
    assert_eq!(CHE::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(CHE::THOUSAND_SEPARATOR, ".");
    assert_eq!(CHE::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_chf_currency() {
    assert_eq!(CHF::CODE, "CHF");
    assert_eq!(CHF::SYMBOL, "₣");
    assert_eq!(CHF::NAME, "Swiss franc");
    assert_eq!(CHF::NUMERIC, 756);
    assert_eq!(CHF::MINOR_UNIT, 2);
    assert_eq!(CHF::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(CHF::THOUSAND_SEPARATOR, ",");
    assert_eq!(CHF::DECIMAL_SEPARATOR, ".");
}

#[test]
fn test_chw_currency() {
    assert_eq!(CHW::CODE, "CHW");
    assert_eq!(CHW::SYMBOL, "¤");
    assert_eq!(CHW::NAME, "WIR Franc");
    assert_eq!(CHW::NUMERIC, 948);
    assert_eq!(CHW::MINOR_UNIT, 2);
    assert_eq!(CHW::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(CHW::THOUSAND_SEPARATOR, ".");
    assert_eq!(CHW::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_clf_currency() {
    assert_eq!(CLF::CODE, "CLF");
    assert_eq!(CLF::SYMBOL, "¤");
    assert_eq!(CLF::NAME, "Unidad de Fomento");
    assert_eq!(CLF::NUMERIC, 990);
    assert_eq!(CLF::MINOR_UNIT, 4);
    assert_eq!(CLF::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(CLF::THOUSAND_SEPARATOR, ".");
    assert_eq!(CLF::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_clp_currency() {
    assert_eq!(CLP::CODE, "CLP");
    assert_eq!(CLP::SYMBOL, "$");
    assert_eq!(CLP::NAME, "Chilean peso");
    assert_eq!(CLP::NUMERIC, 152);
    assert_eq!(CLP::MINOR_UNIT, 0);
    assert_eq!(CLP::MINOR_UNIT_SYMBOL, "");
    assert_eq!(CLP::THOUSAND_SEPARATOR, ".");
    assert_eq!(CLP::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_cny_currency() {
    assert_eq!(CNY::CODE, "CNY");
    assert_eq!(CNY::SYMBOL, "¥");
    assert_eq!(CNY::NAME, "Renminbi (Chinese) yuan");
    assert_eq!(CNY::NUMERIC, 156);
    assert_eq!(CNY::MINOR_UNIT, 2);
    assert_eq!(CNY::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(CNY::THOUSAND_SEPARATOR, ",");
    assert_eq!(CNY::DECIMAL_SEPARATOR, ".");
}

#[test]
fn test_cop_currency() {
    assert_eq!(COP::CODE, "COP");
    assert_eq!(COP::SYMBOL, "$");
    assert_eq!(COP::NAME, "Colombian peso");
    assert_eq!(COP::NUMERIC, 170);
    assert_eq!(COP::MINOR_UNIT, 2);
    assert_eq!(COP::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(COP::THOUSAND_SEPARATOR, ".");
    assert_eq!(COP::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_cou_currency() {
    assert_eq!(COU::CODE, "COU");
    assert_eq!(COU::SYMBOL, "¤");
    assert_eq!(COU::NAME, "Unidad de Valor Real (UVR)");
    assert_eq!(COU::NUMERIC, 970);
    assert_eq!(COU::MINOR_UNIT, 2);
    assert_eq!(COU::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(COU::THOUSAND_SEPARATOR, ".");
    assert_eq!(COU::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_crc_currency() {
    assert_eq!(CRC::CODE, "CRC");
    assert_eq!(CRC::SYMBOL, "₡");
    assert_eq!(CRC::NAME, "Costa Rican colon");
    assert_eq!(CRC::NUMERIC, 188);
    assert_eq!(CRC::MINOR_UNIT, 2);
    assert_eq!(CRC::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(CRC::THOUSAND_SEPARATOR, ",");
    assert_eq!(CRC::DECIMAL_SEPARATOR, ".");
}

#[test]
fn test_cuc_currency() {
    assert_eq!(CUC::CODE, "CUC");
    assert_eq!(CUC::SYMBOL, "$");
    assert_eq!(CUC::NAME, "Cuban convertible peso");
    assert_eq!(CUC::NUMERIC, 931);
    assert_eq!(CUC::MINOR_UNIT, 2);
    assert_eq!(CUC::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(CUC::THOUSAND_SEPARATOR, ".");
    assert_eq!(CUC::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_cup_currency() {
    assert_eq!(CUP::CODE, "CUP");
    assert_eq!(CUP::SYMBOL, "₱");
    assert_eq!(CUP::NAME, "Cuban peso");
    assert_eq!(CUP::NUMERIC, 192);
    assert_eq!(CUP::MINOR_UNIT, 2);
    assert_eq!(CUP::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(CUP::THOUSAND_SEPARATOR, ".");
    assert_eq!(CUP::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_cve_currency() {
    assert_eq!(CVE::CODE, "CVE");
    assert_eq!(CVE::SYMBOL, "Esc");
    assert_eq!(CVE::NAME, "Cape Verdean escudo");
    assert_eq!(CVE::NUMERIC, 132);
    assert_eq!(CVE::MINOR_UNIT, 2);
    assert_eq!(CVE::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(CVE::THOUSAND_SEPARATOR, ".");
    assert_eq!(CVE::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_czk_currency() {
    assert_eq!(CZK::CODE, "CZK");
    assert_eq!(CZK::SYMBOL, "Kč");
    assert_eq!(CZK::NAME, "Czech koruna");
    assert_eq!(CZK::NUMERIC, 203);
    assert_eq!(CZK::MINOR_UNIT, 2);
    assert_eq!(CZK::MINOR_UNIT_SYMBOL, "h");
    assert_eq!(CZK::THOUSAND_SEPARATOR, ".");
    assert_eq!(CZK::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_djf_currency() {
    assert_eq!(DJF::CODE, "DJF");
    assert_eq!(DJF::SYMBOL, "₣");
    assert_eq!(DJF::NAME, "Djiboutian franc");
    assert_eq!(DJF::NUMERIC, 262);
    assert_eq!(DJF::MINOR_UNIT, 0);
    assert_eq!(DJF::MINOR_UNIT_SYMBOL, "");
    assert_eq!(DJF::THOUSAND_SEPARATOR, ".");
    assert_eq!(DJF::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_dkk_currency() {
    assert_eq!(DKK::CODE, "DKK");
    assert_eq!(DKK::SYMBOL, "kr");
    assert_eq!(DKK::NAME, "Danish krone");
    assert_eq!(DKK::NUMERIC, 208);
    assert_eq!(DKK::MINOR_UNIT, 2);
    assert_eq!(DKK::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(DKK::THOUSAND_SEPARATOR, ".");
    assert_eq!(DKK::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_dop_currency() {
    assert_eq!(DOP::CODE, "DOP");
    assert_eq!(DOP::SYMBOL, "RD$");
    assert_eq!(DOP::NAME, "Dominican peso");
    assert_eq!(DOP::NUMERIC, 214);
    assert_eq!(DOP::MINOR_UNIT, 2);
    assert_eq!(DOP::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(DOP::THOUSAND_SEPARATOR, ",");
    assert_eq!(DOP::DECIMAL_SEPARATOR, ".");
}

#[test]
fn test_dzd_currency() {
    assert_eq!(DZD::CODE, "DZD");
    assert_eq!(DZD::SYMBOL, "دج");
    assert_eq!(DZD::NAME, "Algerian dinar");
    assert_eq!(DZD::NUMERIC, 12);
    assert_eq!(DZD::MINOR_UNIT, 2);
    assert_eq!(DZD::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(DZD::THOUSAND_SEPARATOR, ".");
    assert_eq!(DZD::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_egp_currency() {
    assert_eq!(EGP::CODE, "EGP");
    assert_eq!(EGP::SYMBOL, "£");
    assert_eq!(EGP::NAME, "Egyptian pound");
    assert_eq!(EGP::NUMERIC, 818);
    assert_eq!(EGP::MINOR_UNIT, 2);
    assert_eq!(EGP::MINOR_UNIT_SYMBOL, "pt");
    assert_eq!(EGP::THOUSAND_SEPARATOR, ".");
    assert_eq!(EGP::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_ern_currency() {
    assert_eq!(ERN::CODE, "ERN");
    assert_eq!(ERN::SYMBOL, "Nfk");
    assert_eq!(ERN::NAME, "Eritrean nakfa");
    assert_eq!(ERN::NUMERIC, 232);
    assert_eq!(ERN::MINOR_UNIT, 2);
    assert_eq!(ERN::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(ERN::THOUSAND_SEPARATOR, ".");
    assert_eq!(ERN::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_etb_currency() {
    assert_eq!(ETB::CODE, "ETB");
    assert_eq!(ETB::SYMBOL, "Br");
    assert_eq!(ETB::NAME, "Ethiopian birr");
    assert_eq!(ETB::NUMERIC, 230);
    assert_eq!(ETB::MINOR_UNIT, 2);
    assert_eq!(ETB::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(ETB::THOUSAND_SEPARATOR, ".");
    assert_eq!(ETB::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_eur_currency() {
    assert_eq!(EUR::CODE, "EUR");
    assert_eq!(EUR::SYMBOL, "€");
    assert_eq!(EUR::NAME, "Euro");
    assert_eq!(EUR::NUMERIC, 978);
    assert_eq!(EUR::MINOR_UNIT, 2);
    assert_eq!(EUR::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(EUR::THOUSAND_SEPARATOR, ".");
    assert_eq!(EUR::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_fjd_currency() {
    assert_eq!(FJD::CODE, "FJD");
    assert_eq!(FJD::SYMBOL, "FJ$");
    assert_eq!(FJD::NAME, "Fiji dollar");
    assert_eq!(FJD::NUMERIC, 242);
    assert_eq!(FJD::MINOR_UNIT, 2);
    assert_eq!(FJD::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(FJD::THOUSAND_SEPARATOR, ".");
    assert_eq!(FJD::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_fkp_currency() {
    assert_eq!(FKP::CODE, "FKP");
    assert_eq!(FKP::SYMBOL, "£");
    assert_eq!(FKP::NAME, "Falkland Islands pound");
    assert_eq!(FKP::NUMERIC, 238);
    assert_eq!(FKP::MINOR_UNIT, 2);
    assert_eq!(FKP::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(FKP::THOUSAND_SEPARATOR, ".");
    assert_eq!(FKP::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_gbp_currency() {
    assert_eq!(GBP::CODE, "GBP");
    assert_eq!(GBP::SYMBOL, "£");
    assert_eq!(GBP::NAME, "Pound sterling");
    assert_eq!(GBP::NUMERIC, 826);
    assert_eq!(GBP::MINOR_UNIT, 2);
    assert_eq!(GBP::MINOR_UNIT_SYMBOL, "p");
    assert_eq!(GBP::THOUSAND_SEPARATOR, ",");
    assert_eq!(GBP::DECIMAL_SEPARATOR, ".");
}

#[test]
fn test_gel_currency() {
    assert_eq!(GEL::CODE, "GEL");
    assert_eq!(GEL::SYMBOL, "ლ");
    assert_eq!(GEL::NAME, "Georgian lari");
    assert_eq!(GEL::NUMERIC, 981);
    assert_eq!(GEL::MINOR_UNIT, 2);
    assert_eq!(GEL::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(GEL::THOUSAND_SEPARATOR, ".");
    assert_eq!(GEL::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_ghs_currency() {
    assert_eq!(GHS::CODE, "GHS");
    assert_eq!(GHS::SYMBOL, "GH₵");
    assert_eq!(GHS::NAME, "Ghanaian cedi");
    assert_eq!(GHS::NUMERIC, 936);
    assert_eq!(GHS::MINOR_UNIT, 2);
    assert_eq!(GHS::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(GHS::THOUSAND_SEPARATOR, ",");
    assert_eq!(GHS::DECIMAL_SEPARATOR, ".");
}

#[test]
fn test_gip_currency() {
    assert_eq!(GIP::CODE, "GIP");
    assert_eq!(GIP::SYMBOL, "£");
    assert_eq!(GIP::NAME, "Gibraltar pound");
    assert_eq!(GIP::NUMERIC, 292);
    assert_eq!(GIP::MINOR_UNIT, 2);
    assert_eq!(GIP::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(GIP::THOUSAND_SEPARATOR, ".");
    assert_eq!(GIP::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_gmd_currency() {
    assert_eq!(GMD::CODE, "GMD");
    assert_eq!(GMD::SYMBOL, "D");
    assert_eq!(GMD::NAME, "Gambian dalasi");
    assert_eq!(GMD::NUMERIC, 270);
    assert_eq!(GMD::MINOR_UNIT, 2);
    assert_eq!(GMD::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(GMD::THOUSAND_SEPARATOR, ".");
    assert_eq!(GMD::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_gnf_currency() {
    assert_eq!(GNF::CODE, "GNF");
    assert_eq!(GNF::SYMBOL, "₣");
    assert_eq!(GNF::NAME, "Guinean franc");
    assert_eq!(GNF::NUMERIC, 324);
    assert_eq!(GNF::MINOR_UNIT, 0);
    assert_eq!(GNF::MINOR_UNIT_SYMBOL, "");
    assert_eq!(GNF::THOUSAND_SEPARATOR, ".");
    assert_eq!(GNF::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_gtq_currency() {
    assert_eq!(GTQ::CODE, "GTQ");
    assert_eq!(GTQ::SYMBOL, "Q");
    assert_eq!(GTQ::NAME, "Guatemalan quetzal");
    assert_eq!(GTQ::NUMERIC, 320);
    assert_eq!(GTQ::MINOR_UNIT, 2);
    assert_eq!(GTQ::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(GTQ::THOUSAND_SEPARATOR, ",");
    assert_eq!(GTQ::DECIMAL_SEPARATOR, ".");
}

#[test]
fn test_gyd_currency() {
    assert_eq!(GYD::CODE, "GYD");
    assert_eq!(GYD::SYMBOL, "G$");
    assert_eq!(GYD::NAME, "Guyanese dollar");
    assert_eq!(GYD::NUMERIC, 328);
    assert_eq!(GYD::MINOR_UNIT, 2);
    assert_eq!(GYD::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(GYD::THOUSAND_SEPARATOR, ".");
    assert_eq!(GYD::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_hkd_currency() {
    assert_eq!(HKD::CODE, "HKD");
    assert_eq!(HKD::SYMBOL, "HK$");
    assert_eq!(HKD::NAME, "Hong Kong dollar");
    assert_eq!(HKD::NUMERIC, 344);
    assert_eq!(HKD::MINOR_UNIT, 2);
    assert_eq!(HKD::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(HKD::THOUSAND_SEPARATOR, ",");
    assert_eq!(HKD::DECIMAL_SEPARATOR, ".");
}

#[test]
fn test_hnl_currency() {
    assert_eq!(HNL::CODE, "HNL");
    assert_eq!(HNL::SYMBOL, "L");
    assert_eq!(HNL::NAME, "Honduran lempira");
    assert_eq!(HNL::NUMERIC, 340);
    assert_eq!(HNL::MINOR_UNIT, 2);
    assert_eq!(HNL::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(HNL::THOUSAND_SEPARATOR, ",");
    assert_eq!(HNL::DECIMAL_SEPARATOR, ".");
}

#[test]
fn test_hrk_currency() {
    assert_eq!(HRK::CODE, "HRK");
    assert_eq!(HRK::SYMBOL, "kn");
    assert_eq!(HRK::NAME, "Croatian kuna");
    assert_eq!(HRK::NUMERIC, 191);
    assert_eq!(HRK::MINOR_UNIT, 2);
    assert_eq!(HRK::MINOR_UNIT_SYMBOL, "lp");
    assert_eq!(HRK::THOUSAND_SEPARATOR, ".");
    assert_eq!(HRK::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_htg_currency() {
    assert_eq!(HTG::CODE, "HTG");
    assert_eq!(HTG::SYMBOL, "G");
    assert_eq!(HTG::NAME, "Haitian gourde");
    assert_eq!(HTG::NUMERIC, 332);
    assert_eq!(HTG::MINOR_UNIT, 2);
    assert_eq!(HTG::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(HTG::THOUSAND_SEPARATOR, ".");
    assert_eq!(HTG::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_huf_currency() {
    assert_eq!(HUF::CODE, "HUF");
    assert_eq!(HUF::SYMBOL, "Ft");
    assert_eq!(HUF::NAME, "Hungarian forint");
    assert_eq!(HUF::NUMERIC, 348);
    assert_eq!(HUF::MINOR_UNIT, 2);
    assert_eq!(HUF::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(HUF::THOUSAND_SEPARATOR, ".");
    assert_eq!(HUF::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_idr_currency() {
    assert_eq!(IDR::CODE, "IDR");
    assert_eq!(IDR::SYMBOL, "Rp");
    assert_eq!(IDR::NAME, "Indonesian rupiah");
    assert_eq!(IDR::NUMERIC, 360);
    assert_eq!(IDR::MINOR_UNIT, 2);
    assert_eq!(IDR::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(IDR::THOUSAND_SEPARATOR, ".");
    assert_eq!(IDR::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_ils_currency() {
    assert_eq!(ILS::CODE, "ILS");
    assert_eq!(ILS::SYMBOL, "₪");
    assert_eq!(ILS::NAME, "Israeli new shekel");
    assert_eq!(ILS::NUMERIC, 376);
    assert_eq!(ILS::MINOR_UNIT, 2);
    assert_eq!(ILS::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(ILS::THOUSAND_SEPARATOR, ",");
    assert_eq!(ILS::DECIMAL_SEPARATOR, ".");
}

#[test]
fn test_inr_currency() {
    assert_eq!(INR::CODE, "INR");
    assert_eq!(INR::SYMBOL, "₹");
    assert_eq!(INR::NAME, "Indian rupee");
    assert_eq!(INR::NUMERIC, 356);
    assert_eq!(INR::MINOR_UNIT, 2);
    assert_eq!(INR::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(INR::THOUSAND_SEPARATOR, ",");
    assert_eq!(INR::DECIMAL_SEPARATOR, ".");
}

#[test]
fn test_iqd_currency() {
    assert_eq!(IQD::CODE, "IQD");
    assert_eq!(IQD::SYMBOL, "د.ع");
    assert_eq!(IQD::NAME, "Iraqi dinar");
    assert_eq!(IQD::NUMERIC, 368);
    assert_eq!(IQD::MINOR_UNIT, 3);
    assert_eq!(IQD::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(IQD::THOUSAND_SEPARATOR, ".");
    assert_eq!(IQD::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_irr_currency() {
    assert_eq!(IRR::CODE, "IRR");
    assert_eq!(IRR::SYMBOL, "﷼");
    assert_eq!(IRR::NAME, "Iranian rial");
    assert_eq!(IRR::NUMERIC, 364);
    assert_eq!(IRR::MINOR_UNIT, 2);
    assert_eq!(IRR::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(IRR::THOUSAND_SEPARATOR, ".");
    assert_eq!(IRR::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_isk_currency() {
    assert_eq!(ISK::CODE, "ISK");
    assert_eq!(ISK::SYMBOL, "kr");
    assert_eq!(ISK::NAME, "Icelandic króna");
    assert_eq!(ISK::NUMERIC, 352);
    assert_eq!(ISK::MINOR_UNIT, 0);
    assert_eq!(ISK::MINOR_UNIT_SYMBOL, "");
    assert_eq!(ISK::THOUSAND_SEPARATOR, ",");
    assert_eq!(ISK::DECIMAL_SEPARATOR, ".");
}

#[test]
fn test_jmd_currency() {
    assert_eq!(JMD::CODE, "JMD");
    assert_eq!(JMD::SYMBOL, "$");
    assert_eq!(JMD::NAME, "Jamaican dollar");
    assert_eq!(JMD::NUMERIC, 388);
    assert_eq!(JMD::MINOR_UNIT, 2);
    assert_eq!(JMD::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(JMD::THOUSAND_SEPARATOR, ".");
    assert_eq!(JMD::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_jod_currency() {
    assert_eq!(JOD::CODE, "JOD");
    assert_eq!(JOD::SYMBOL, "JD");
    assert_eq!(JOD::NAME, "Jordanian dinar");
    assert_eq!(JOD::NUMERIC, 400);
    assert_eq!(JOD::MINOR_UNIT, 3);
    assert_eq!(JOD::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(JOD::THOUSAND_SEPARATOR, ".");
    assert_eq!(JOD::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_jpy_currency() {
    assert_eq!(JPY::CODE, "JPY");
    assert_eq!(JPY::SYMBOL, "¥");
    assert_eq!(JPY::NAME, "Japanese yen");
    assert_eq!(JPY::NUMERIC, 392);
    assert_eq!(JPY::MINOR_UNIT, 0);
    assert_eq!(JPY::MINOR_UNIT_SYMBOL, "");
    assert_eq!(JPY::THOUSAND_SEPARATOR, ",");
    assert_eq!(JPY::DECIMAL_SEPARATOR, ".");
}

#[test]
fn test_kes_currency() {
    assert_eq!(KES::CODE, "KES");
    assert_eq!(KES::SYMBOL, "Ksh");
    assert_eq!(KES::NAME, "Kenyan shilling");
    assert_eq!(KES::NUMERIC, 404);
    assert_eq!(KES::MINOR_UNIT, 2);
    assert_eq!(KES::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(KES::THOUSAND_SEPARATOR, ",");
    assert_eq!(KES::DECIMAL_SEPARATOR, ".");
}

#[test]
fn test_kgs_currency() {
    assert_eq!(KGS::CODE, "KGS");
    assert_eq!(KGS::SYMBOL, "С̲");
    assert_eq!(KGS::NAME, "Kyrgyzstani som");
    assert_eq!(KGS::NUMERIC, 417);
    assert_eq!(KGS::MINOR_UNIT, 2);
    assert_eq!(KGS::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(KGS::THOUSAND_SEPARATOR, ".");
    assert_eq!(KGS::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_khr_currency() {
    assert_eq!(KHR::CODE, "KHR");
    assert_eq!(KHR::SYMBOL, "៛");
    assert_eq!(KHR::NAME, "Cambodian riel");
    assert_eq!(KHR::NUMERIC, 116);
    assert_eq!(KHR::MINOR_UNIT, 2);
    assert_eq!(KHR::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(KHR::THOUSAND_SEPARATOR, ".");
    assert_eq!(KHR::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_kmf_currency() {
    assert_eq!(KMF::CODE, "KMF");
    assert_eq!(KMF::SYMBOL, "₣");
    assert_eq!(KMF::NAME, "Comoro franc");
    assert_eq!(KMF::NUMERIC, 174);
    assert_eq!(KMF::MINOR_UNIT, 0);
    assert_eq!(KMF::MINOR_UNIT_SYMBOL, "");
    assert_eq!(KMF::THOUSAND_SEPARATOR, ".");
    assert_eq!(KMF::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_kpw_currency() {
    assert_eq!(KPW::CODE, "KPW");
    assert_eq!(KPW::SYMBOL, "₩");
    assert_eq!(KPW::NAME, "North Korean won");
    assert_eq!(KPW::NUMERIC, 408);
    assert_eq!(KPW::MINOR_UNIT, 2);
    assert_eq!(KPW::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(KPW::THOUSAND_SEPARATOR, ".");
    assert_eq!(KPW::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_krw_currency() {
    assert_eq!(KRW::CODE, "KRW");
    assert_eq!(KRW::SYMBOL, "₩");
    assert_eq!(KRW::NAME, "South Korean won");
    assert_eq!(KRW::NUMERIC, 410);
    assert_eq!(KRW::MINOR_UNIT, 0);
    assert_eq!(KRW::MINOR_UNIT_SYMBOL, "");
    assert_eq!(KRW::THOUSAND_SEPARATOR, ",");
    assert_eq!(KRW::DECIMAL_SEPARATOR, ".");
}

#[test]
fn test_kwd_currency() {
    assert_eq!(KWD::CODE, "KWD");
    assert_eq!(KWD::SYMBOL, "د.ك");
    assert_eq!(KWD::NAME, "Kuwaiti dinar");
    assert_eq!(KWD::NUMERIC, 414);
    assert_eq!(KWD::MINOR_UNIT, 3);
    assert_eq!(KWD::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(KWD::THOUSAND_SEPARATOR, ".");
    assert_eq!(KWD::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_kyd_currency() {
    assert_eq!(KYD::CODE, "KYD");
    assert_eq!(KYD::SYMBOL, "$");
    assert_eq!(KYD::NAME, "Cayman Islands dollar");
    assert_eq!(KYD::NUMERIC, 136);
    assert_eq!(KYD::MINOR_UNIT, 2);
    assert_eq!(KYD::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(KYD::THOUSAND_SEPARATOR, ".");
    assert_eq!(KYD::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_kzt_currency() {
    assert_eq!(KZT::CODE, "KZT");
    assert_eq!(KZT::SYMBOL, "₸");
    assert_eq!(KZT::NAME, "Kazakhstani tenge");
    assert_eq!(KZT::NUMERIC, 398);
    assert_eq!(KZT::MINOR_UNIT, 2);
    assert_eq!(KZT::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(KZT::THOUSAND_SEPARATOR, ".");
    assert_eq!(KZT::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_lak_currency() {
    assert_eq!(LAK::CODE, "LAK");
    assert_eq!(LAK::SYMBOL, "₭");
    assert_eq!(LAK::NAME, "Lao kip");
    assert_eq!(LAK::NUMERIC, 418);
    assert_eq!(LAK::MINOR_UNIT, 2);
    assert_eq!(LAK::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(LAK::THOUSAND_SEPARATOR, ".");
    assert_eq!(LAK::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_lbp_currency() {
    assert_eq!(LBP::CODE, "LBP");
    assert_eq!(LBP::SYMBOL, "LL");
    assert_eq!(LBP::NAME, "Lebanese pound");
    assert_eq!(LBP::NUMERIC, 422);
    assert_eq!(LBP::MINOR_UNIT, 2);
    assert_eq!(LBP::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(LBP::THOUSAND_SEPARATOR, ".");
    assert_eq!(LBP::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_lkr_currency() {
    assert_eq!(LKR::CODE, "LKR");
    assert_eq!(LKR::SYMBOL, "₨");
    assert_eq!(LKR::NAME, "Sri Lankan rupee");
    assert_eq!(LKR::NUMERIC, 144);
    assert_eq!(LKR::MINOR_UNIT, 2);
    assert_eq!(LKR::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(LKR::THOUSAND_SEPARATOR, ",");
    assert_eq!(LKR::DECIMAL_SEPARATOR, ".");
}

#[test]
fn test_lrd_currency() {
    assert_eq!(LRD::CODE, "LRD");
    assert_eq!(LRD::SYMBOL, "L$");
    assert_eq!(LRD::NAME, "Liberian dollar");
    assert_eq!(LRD::NUMERIC, 430);
    assert_eq!(LRD::MINOR_UNIT, 2);
    assert_eq!(LRD::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(LRD::THOUSAND_SEPARATOR, ".");
    assert_eq!(LRD::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_lsl_currency() {
    assert_eq!(LSL::CODE, "LSL");
    assert_eq!(LSL::SYMBOL, "M");
    assert_eq!(LSL::NAME, "Lesotho loti");
    assert_eq!(LSL::NUMERIC, 426);
    assert_eq!(LSL::MINOR_UNIT, 2);
    assert_eq!(LSL::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(LSL::THOUSAND_SEPARATOR, ".");
    assert_eq!(LSL::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_lyd_currency() {
    assert_eq!(LYD::CODE, "LYD");
    assert_eq!(LYD::SYMBOL, "ل.د");
    assert_eq!(LYD::NAME, "Libyan dinar");
    assert_eq!(LYD::NUMERIC, 434);
    assert_eq!(LYD::MINOR_UNIT, 3);
    assert_eq!(LYD::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(LYD::THOUSAND_SEPARATOR, ".");
    assert_eq!(LYD::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_mad_currency() {
    assert_eq!(MAD::CODE, "MAD");
    assert_eq!(MAD::SYMBOL, "د.م.");
    assert_eq!(MAD::NAME, "Moroccan dirham");
    assert_eq!(MAD::NUMERIC, 504);
    assert_eq!(MAD::MINOR_UNIT, 2);
    assert_eq!(MAD::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(MAD::THOUSAND_SEPARATOR, ".");
    assert_eq!(MAD::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_mdl_currency() {
    assert_eq!(MDL::CODE, "MDL");
    assert_eq!(MDL::SYMBOL, "¤");
    assert_eq!(MDL::NAME, "Moldovan leu");
    assert_eq!(MDL::NUMERIC, 498);
    assert_eq!(MDL::MINOR_UNIT, 2);
    assert_eq!(MDL::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(MDL::THOUSAND_SEPARATOR, ".");
    assert_eq!(MDL::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_mga_currency() {
    assert_eq!(MGA::CODE, "MGA");
    assert_eq!(MGA::SYMBOL, "Ar");
    assert_eq!(MGA::NAME, "Malagasy ariary");
    assert_eq!(MGA::NUMERIC, 969);
    assert_eq!(MGA::MINOR_UNIT, 2);
    assert_eq!(MGA::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(MGA::THOUSAND_SEPARATOR, ".");
    assert_eq!(MGA::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_mkd_currency() {
    assert_eq!(MKD::CODE, "MKD");
    assert_eq!(MKD::SYMBOL, "ден");
    assert_eq!(MKD::NAME, "Macedonian denar");
    assert_eq!(MKD::NUMERIC, 807);
    assert_eq!(MKD::MINOR_UNIT, 2);
    assert_eq!(MKD::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(MKD::THOUSAND_SEPARATOR, ".");
    assert_eq!(MKD::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_mmk_currency() {
    assert_eq!(MMK::CODE, "MMK");
    assert_eq!(MMK::SYMBOL, "K");
    assert_eq!(MMK::NAME, "Myanmar kyat");
    assert_eq!(MMK::NUMERIC, 104);
    assert_eq!(MMK::MINOR_UNIT, 2);
    assert_eq!(MMK::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(MMK::THOUSAND_SEPARATOR, ".");
    assert_eq!(MMK::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_mnt_currency() {
    assert_eq!(MNT::CODE, "MNT");
    assert_eq!(MNT::SYMBOL, "₮");
    assert_eq!(MNT::NAME, "Mongolian tögrög");
    assert_eq!(MNT::NUMERIC, 496);
    assert_eq!(MNT::MINOR_UNIT, 2);
    assert_eq!(MNT::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(MNT::THOUSAND_SEPARATOR, ".");
    assert_eq!(MNT::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_mop_currency() {
    assert_eq!(MOP::CODE, "MOP");
    assert_eq!(MOP::SYMBOL, "MOP$");
    assert_eq!(MOP::NAME, "Macanese pataca");
    assert_eq!(MOP::NUMERIC, 446);
    assert_eq!(MOP::MINOR_UNIT, 2);
    assert_eq!(MOP::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(MOP::THOUSAND_SEPARATOR, ".");
    assert_eq!(MOP::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_mru_currency() {
    assert_eq!(MRU::CODE, "MRU");
    assert_eq!(MRU::SYMBOL, "UM");
    assert_eq!(MRU::NAME, "Mauritanian ouguiya");
    assert_eq!(MRU::NUMERIC, 929);
    assert_eq!(MRU::MINOR_UNIT, 2);
    assert_eq!(MRU::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(MRU::THOUSAND_SEPARATOR, ".");
    assert_eq!(MRU::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_mur_currency() {
    assert_eq!(MUR::CODE, "MUR");
    assert_eq!(MUR::SYMBOL, "₨");
    assert_eq!(MUR::NAME, "Mauritian rupee");
    assert_eq!(MUR::NUMERIC, 480);
    assert_eq!(MUR::MINOR_UNIT, 2);
    assert_eq!(MUR::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(MUR::THOUSAND_SEPARATOR, ".");
    assert_eq!(MUR::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_mvr_currency() {
    assert_eq!(MVR::CODE, "MVR");
    assert_eq!(MVR::SYMBOL, "Rf.");
    assert_eq!(MVR::NAME, "Maldivian rufiyaa");
    assert_eq!(MVR::NUMERIC, 462);
    assert_eq!(MVR::MINOR_UNIT, 2);
    assert_eq!(MVR::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(MVR::THOUSAND_SEPARATOR, ".");
    assert_eq!(MVR::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_mwk_currency() {
    assert_eq!(MWK::CODE, "MWK");
    assert_eq!(MWK::SYMBOL, "K");
    assert_eq!(MWK::NAME, "Malawian kwacha");
    assert_eq!(MWK::NUMERIC, 454);
    assert_eq!(MWK::MINOR_UNIT, 2);
    assert_eq!(MWK::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(MWK::THOUSAND_SEPARATOR, ".");
    assert_eq!(MWK::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_mxn_currency() {
    assert_eq!(MXN::CODE, "MXN");
    assert_eq!(MXN::SYMBOL, "$");
    assert_eq!(MXN::NAME, "Mexican peso");
    assert_eq!(MXN::NUMERIC, 484);
    assert_eq!(MXN::MINOR_UNIT, 2);
    assert_eq!(MXN::MINOR_UNIT_SYMBOL, "¢");
    assert_eq!(MXN::THOUSAND_SEPARATOR, ",");
    assert_eq!(MXN::DECIMAL_SEPARATOR, ".");
}

#[test]
fn test_mxv_currency() {
    assert_eq!(MXV::CODE, "MXV");
    assert_eq!(MXV::SYMBOL, "¤");
    assert_eq!(MXV::NAME, "Mexican Unidad de Inversion (UDI)");
    assert_eq!(MXV::NUMERIC, 979);
    assert_eq!(MXV::MINOR_UNIT, 2);
    assert_eq!(MXV::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(MXV::THOUSAND_SEPARATOR, ".");
    assert_eq!(MXV::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_myr_currency() {
    assert_eq!(MYR::CODE, "MYR");
    assert_eq!(MYR::SYMBOL, "RM");
    assert_eq!(MYR::NAME, "Malaysian ringgit");
    assert_eq!(MYR::NUMERIC, 458);
    assert_eq!(MYR::MINOR_UNIT, 2);
    assert_eq!(MYR::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(MYR::THOUSAND_SEPARATOR, ",");
    assert_eq!(MYR::DECIMAL_SEPARATOR, ".");
}

#[test]
fn test_mzn_currency() {
    assert_eq!(MZN::CODE, "MZN");
    assert_eq!(MZN::SYMBOL, "MT");
    assert_eq!(MZN::NAME, "Mozambican metical");
    assert_eq!(MZN::NUMERIC, 943);
    assert_eq!(MZN::MINOR_UNIT, 2);
    assert_eq!(MZN::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(MZN::THOUSAND_SEPARATOR, ".");
    assert_eq!(MZN::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_nad_currency() {
    assert_eq!(NAD::CODE, "NAD");
    assert_eq!(NAD::SYMBOL, "N$");
    assert_eq!(NAD::NAME, "Namibian dollar");
    assert_eq!(NAD::NUMERIC, 516);
    assert_eq!(NAD::MINOR_UNIT, 2);
    assert_eq!(NAD::MINOR_UNIT_SYMBOL, "NA");
    assert_eq!(NAD::THOUSAND_SEPARATOR, ".");
    assert_eq!(NAD::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_ngn_currency() {
    assert_eq!(NGN::CODE, "NGN");
    assert_eq!(NGN::SYMBOL, "₦");
    assert_eq!(NGN::NAME, "Nigerian naira");
    assert_eq!(NGN::NUMERIC, 566);
    assert_eq!(NGN::MINOR_UNIT, 2);
    assert_eq!(NGN::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(NGN::THOUSAND_SEPARATOR, ",");
    assert_eq!(NGN::DECIMAL_SEPARATOR, ".");
}

#[test]
fn test_nio_currency() {
    assert_eq!(NIO::CODE, "NIO");
    assert_eq!(NIO::SYMBOL, "C$");
    assert_eq!(NIO::NAME, "Nicaraguan córdoba");
    assert_eq!(NIO::NUMERIC, 558);
    assert_eq!(NIO::MINOR_UNIT, 2);
    assert_eq!(NIO::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(NIO::THOUSAND_SEPARATOR, ",");
    assert_eq!(NIO::DECIMAL_SEPARATOR, ".");
}

#[test]
fn test_nok_currency() {
    assert_eq!(NOK::CODE, "NOK");
    assert_eq!(NOK::SYMBOL, "kr");
    assert_eq!(NOK::NAME, "Norwegian krone");
    assert_eq!(NOK::NUMERIC, 578);
    assert_eq!(NOK::MINOR_UNIT, 2);
    assert_eq!(NOK::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(NOK::THOUSAND_SEPARATOR, ".");
    assert_eq!(NOK::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_npr_currency() {
    assert_eq!(NPR::CODE, "NPR");
    assert_eq!(NPR::SYMBOL, "₨");
    assert_eq!(NPR::NAME, "Nepalese rupee");
    assert_eq!(NPR::NUMERIC, 524);
    assert_eq!(NPR::MINOR_UNIT, 2);
    assert_eq!(NPR::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(NPR::THOUSAND_SEPARATOR, ",");
    assert_eq!(NPR::DECIMAL_SEPARATOR, ".");
}

#[test]
fn test_nzd_currency() {
    assert_eq!(NZD::CODE, "NZD");
    assert_eq!(NZD::SYMBOL, "$");
    assert_eq!(NZD::NAME, "New Zealand dollar");
    assert_eq!(NZD::NUMERIC, 554);
    assert_eq!(NZD::MINOR_UNIT, 2);
    assert_eq!(NZD::MINOR_UNIT_SYMBOL, "c");
    assert_eq!(NZD::THOUSAND_SEPARATOR, ",");
    assert_eq!(NZD::DECIMAL_SEPARATOR, ".");
}

#[test]
fn test_omr_currency() {
    assert_eq!(OMR::CODE, "OMR");
    assert_eq!(OMR::SYMBOL, "ر.ع.");
    assert_eq!(OMR::NAME, "Omani rial");
    assert_eq!(OMR::NUMERIC, 512);
    assert_eq!(OMR::MINOR_UNIT, 3);
    assert_eq!(OMR::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(OMR::THOUSAND_SEPARATOR, ".");
    assert_eq!(OMR::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_pab_currency() {
    assert_eq!(PAB::CODE, "PAB");
    assert_eq!(PAB::SYMBOL, "B/.");
    assert_eq!(PAB::NAME, "Panamanian balboa");
    assert_eq!(PAB::NUMERIC, 590);
    assert_eq!(PAB::MINOR_UNIT, 2);
    assert_eq!(PAB::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(PAB::THOUSAND_SEPARATOR, ",");
    assert_eq!(PAB::DECIMAL_SEPARATOR, ".");
}

#[test]
fn test_pen_currency() {
    assert_eq!(PEN::CODE, "PEN");
    assert_eq!(PEN::SYMBOL, "S/");
    assert_eq!(PEN::NAME, "Peruvian sol");
    assert_eq!(PEN::NUMERIC, 604);
    assert_eq!(PEN::MINOR_UNIT, 2);
    assert_eq!(PEN::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(PEN::THOUSAND_SEPARATOR, ".");
    assert_eq!(PEN::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_pgk_currency() {
    assert_eq!(PGK::CODE, "PGK");
    assert_eq!(PGK::SYMBOL, "K");
    assert_eq!(PGK::NAME, "Papua New Guinean kina");
    assert_eq!(PGK::NUMERIC, 598);
    assert_eq!(PGK::MINOR_UNIT, 2);
    assert_eq!(PGK::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(PGK::THOUSAND_SEPARATOR, ".");
    assert_eq!(PGK::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_php_currency() {
    assert_eq!(PHP::CODE, "PHP");
    assert_eq!(PHP::SYMBOL, "₱");
    assert_eq!(PHP::NAME, "Philippine peso");
    assert_eq!(PHP::NUMERIC, 608);
    assert_eq!(PHP::MINOR_UNIT, 2);
    assert_eq!(PHP::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(PHP::THOUSAND_SEPARATOR, ",");
    assert_eq!(PHP::DECIMAL_SEPARATOR, ".");
}

#[test]
fn test_pkr_currency() {
    assert_eq!(PKR::CODE, "PKR");
    assert_eq!(PKR::SYMBOL, "₨");
    assert_eq!(PKR::NAME, "Pakistani rupee");
    assert_eq!(PKR::NUMERIC, 586);
    assert_eq!(PKR::MINOR_UNIT, 2);
    assert_eq!(PKR::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(PKR::THOUSAND_SEPARATOR, ",");
    assert_eq!(PKR::DECIMAL_SEPARATOR, ".");
}

#[test]
fn test_pln_currency() {
    assert_eq!(PLN::CODE, "PLN");
    assert_eq!(PLN::SYMBOL, "zł");
    assert_eq!(PLN::NAME, "Polish złoty");
    assert_eq!(PLN::NUMERIC, 985);
    assert_eq!(PLN::MINOR_UNIT, 2);
    assert_eq!(PLN::MINOR_UNIT_SYMBOL, "gr");
    assert_eq!(PLN::THOUSAND_SEPARATOR, ".");
    assert_eq!(PLN::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_pyg_currency() {
    assert_eq!(PYG::CODE, "PYG");
    assert_eq!(PYG::SYMBOL, "₲");
    assert_eq!(PYG::NAME, "Paraguayan guaraní");
    assert_eq!(PYG::NUMERIC, 600);
    assert_eq!(PYG::MINOR_UNIT, 0);
    assert_eq!(PYG::MINOR_UNIT_SYMBOL, "");
    assert_eq!(PYG::THOUSAND_SEPARATOR, ".");
    assert_eq!(PYG::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_qar_currency() {
    assert_eq!(QAR::CODE, "QAR");
    assert_eq!(QAR::SYMBOL, "ر.ق");
    assert_eq!(QAR::NAME, "Qatari riyal");
    assert_eq!(QAR::NUMERIC, 634);
    assert_eq!(QAR::MINOR_UNIT, 2);
    assert_eq!(QAR::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(QAR::THOUSAND_SEPARATOR, ".");
    assert_eq!(QAR::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_ron_currency() {
    assert_eq!(RON::CODE, "RON");
    assert_eq!(RON::SYMBOL, "L");
    assert_eq!(RON::NAME, "Romanian leu");
    assert_eq!(RON::NUMERIC, 946);
    assert_eq!(RON::MINOR_UNIT, 2);
    assert_eq!(RON::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(RON::THOUSAND_SEPARATOR, ".");
    assert_eq!(RON::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_rsd_currency() {
    assert_eq!(RSD::CODE, "RSD");
    assert_eq!(RSD::SYMBOL, "дин");
    assert_eq!(RSD::NAME, "Serbian dinar");
    assert_eq!(RSD::NUMERIC, 941);
    assert_eq!(RSD::MINOR_UNIT, 2);
    assert_eq!(RSD::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(RSD::THOUSAND_SEPARATOR, ".");
    assert_eq!(RSD::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_rub_currency() {
    assert_eq!(RUB::CODE, "RUB");
    assert_eq!(RUB::SYMBOL, "₽");
    assert_eq!(RUB::NAME, "Russian ruble");
    assert_eq!(RUB::NUMERIC, 643);
    assert_eq!(RUB::MINOR_UNIT, 2);
    assert_eq!(RUB::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(RUB::THOUSAND_SEPARATOR, ".");
    assert_eq!(RUB::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_rwf_currency() {
    assert_eq!(RWF::CODE, "RWF");
    assert_eq!(RWF::SYMBOL, "FRw");
    assert_eq!(RWF::NAME, "Rwandan franc");
    assert_eq!(RWF::NUMERIC, 646);
    assert_eq!(RWF::MINOR_UNIT, 0);
    assert_eq!(RWF::MINOR_UNIT_SYMBOL, "");
    assert_eq!(RWF::THOUSAND_SEPARATOR, ".");
    assert_eq!(RWF::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_sar_currency() {
    assert_eq!(SAR::CODE, "SAR");
    assert_eq!(SAR::SYMBOL, "ر.س");
    assert_eq!(SAR::NAME, "Saudi riyal");
    assert_eq!(SAR::NUMERIC, 682);
    assert_eq!(SAR::MINOR_UNIT, 2);
    assert_eq!(SAR::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(SAR::THOUSAND_SEPARATOR, ",");
    assert_eq!(SAR::DECIMAL_SEPARATOR, ".");
}

#[test]
fn test_sbd_currency() {
    assert_eq!(SBD::CODE, "SBD");
    assert_eq!(SBD::SYMBOL, "S$");
    assert_eq!(SBD::NAME, "Solomon Islands dollar");
    assert_eq!(SBD::NUMERIC, 90);
    assert_eq!(SBD::MINOR_UNIT, 2);
    assert_eq!(SBD::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(SBD::THOUSAND_SEPARATOR, ".");
    assert_eq!(SBD::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_scr_currency() {
    assert_eq!(SCR::CODE, "SCR");
    assert_eq!(SCR::SYMBOL, "SRe");
    assert_eq!(SCR::NAME, "Seychelles rupee");
    assert_eq!(SCR::NUMERIC, 690);
    assert_eq!(SCR::MINOR_UNIT, 2);
    assert_eq!(SCR::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(SCR::THOUSAND_SEPARATOR, ".");
    assert_eq!(SCR::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_sdg_currency() {
    assert_eq!(SDG::CODE, "SDG");
    assert_eq!(SDG::SYMBOL, "¤");
    assert_eq!(SDG::NAME, "Sudanese pound");
    assert_eq!(SDG::NUMERIC, 938);
    assert_eq!(SDG::MINOR_UNIT, 2);
    assert_eq!(SDG::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(SDG::THOUSAND_SEPARATOR, ".");
    assert_eq!(SDG::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_sek_currency() {
    assert_eq!(SEK::CODE, "SEK");
    assert_eq!(SEK::SYMBOL, "kr");
    assert_eq!(SEK::NAME, "Swedish krona/kronor");
    assert_eq!(SEK::NUMERIC, 752);
    assert_eq!(SEK::MINOR_UNIT, 2);
    assert_eq!(SEK::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(SEK::THOUSAND_SEPARATOR, ".");
    assert_eq!(SEK::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_sgd_currency() {
    assert_eq!(SGD::CODE, "SGD");
    assert_eq!(SGD::SYMBOL, "S$");
    assert_eq!(SGD::NAME, "Singapore dollar");
    assert_eq!(SGD::NUMERIC, 702);
    assert_eq!(SGD::MINOR_UNIT, 2);
    assert_eq!(SGD::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(SGD::THOUSAND_SEPARATOR, ",");
    assert_eq!(SGD::DECIMAL_SEPARATOR, ".");
}

#[test]
fn test_shp_currency() {
    assert_eq!(SHP::CODE, "SHP");
    assert_eq!(SHP::SYMBOL, "£");
    assert_eq!(SHP::NAME, "Saint Helena pound");
    assert_eq!(SHP::NUMERIC, 654);
    assert_eq!(SHP::MINOR_UNIT, 2);
    assert_eq!(SHP::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(SHP::THOUSAND_SEPARATOR, ".");
    assert_eq!(SHP::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_sle_currency() {
    assert_eq!(SLE::CODE, "SLE");
    assert_eq!(SLE::SYMBOL, "Le");
    assert_eq!(SLE::NAME, "Sierra Leonean leone");
    assert_eq!(SLE::NUMERIC, 925);
    assert_eq!(SLE::MINOR_UNIT, 2);
    assert_eq!(SLE::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(SLE::THOUSAND_SEPARATOR, ".");
    assert_eq!(SLE::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_sll_currency() {
    assert_eq!(SLL::CODE, "SLL");
    assert_eq!(SLL::SYMBOL, "Le");
    assert_eq!(SLL::NAME, "Sierra Leonean leone");
    assert_eq!(SLL::NUMERIC, 694);
    assert_eq!(SLL::MINOR_UNIT, 2);
    assert_eq!(SLL::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(SLL::THOUSAND_SEPARATOR, ".");
    assert_eq!(SLL::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_sos_currency() {
    assert_eq!(SOS::CODE, "SOS");
    assert_eq!(SOS::SYMBOL, "Sh.So.");
    assert_eq!(SOS::NAME, "Somali shilling");
    assert_eq!(SOS::NUMERIC, 706);
    assert_eq!(SOS::MINOR_UNIT, 2);
    assert_eq!(SOS::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(SOS::THOUSAND_SEPARATOR, ".");
    assert_eq!(SOS::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_srd_currency() {
    assert_eq!(SRD::CODE, "SRD");
    assert_eq!(SRD::SYMBOL, "$");
    assert_eq!(SRD::NAME, "Surinamese dollar");
    assert_eq!(SRD::NUMERIC, 968);
    assert_eq!(SRD::MINOR_UNIT, 2);
    assert_eq!(SRD::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(SRD::THOUSAND_SEPARATOR, ".");
    assert_eq!(SRD::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_ssp_currency() {
    assert_eq!(SSP::CODE, "SSP");
    assert_eq!(SSP::SYMBOL, "¤");
    assert_eq!(SSP::NAME, "South Sudanese pound");
    assert_eq!(SSP::NUMERIC, 728);
    assert_eq!(SSP::MINOR_UNIT, 2);
    assert_eq!(SSP::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(SSP::THOUSAND_SEPARATOR, ".");
    assert_eq!(SSP::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_stn_currency() {
    assert_eq!(STN::CODE, "STN");
    assert_eq!(STN::SYMBOL, "Db");
    assert_eq!(STN::NAME, "São Tomé and Príncipe dobra");
    assert_eq!(STN::NUMERIC, 930);
    assert_eq!(STN::MINOR_UNIT, 2);
    assert_eq!(STN::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(STN::THOUSAND_SEPARATOR, ".");
    assert_eq!(STN::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_svc_currency() {
    assert_eq!(SVC::CODE, "SVC");
    assert_eq!(SVC::SYMBOL, "¤");
    assert_eq!(SVC::NAME, "Salvadoran colón");
    assert_eq!(SVC::NUMERIC, 222);
    assert_eq!(SVC::MINOR_UNIT, 2);
    assert_eq!(SVC::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(SVC::THOUSAND_SEPARATOR, ".");
    assert_eq!(SVC::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_syp_currency() {
    assert_eq!(SYP::CODE, "SYP");
    assert_eq!(SYP::SYMBOL, "LS");
    assert_eq!(SYP::NAME, "Syrian pound");
    assert_eq!(SYP::NUMERIC, 760);
    assert_eq!(SYP::MINOR_UNIT, 2);
    assert_eq!(SYP::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(SYP::THOUSAND_SEPARATOR, ".");
    assert_eq!(SYP::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_szl_currency() {
    assert_eq!(SZL::CODE, "SZL");
    assert_eq!(SZL::SYMBOL, "E");
    assert_eq!(SZL::NAME, "Swazi lilangeni");
    assert_eq!(SZL::NUMERIC, 748);
    assert_eq!(SZL::MINOR_UNIT, 2);
    assert_eq!(SZL::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(SZL::THOUSAND_SEPARATOR, ".");
    assert_eq!(SZL::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_thb_currency() {
    assert_eq!(THB::CODE, "THB");
    assert_eq!(THB::SYMBOL, "฿");
    assert_eq!(THB::NAME, "Thai baht");
    assert_eq!(THB::NUMERIC, 764);
    assert_eq!(THB::MINOR_UNIT, 2);
    assert_eq!(THB::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(THB::THOUSAND_SEPARATOR, ",");
    assert_eq!(THB::DECIMAL_SEPARATOR, ".");
}

#[test]
fn test_tjs_currency() {
    assert_eq!(TJS::CODE, "TJS");
    assert_eq!(TJS::SYMBOL, "¤");
    assert_eq!(TJS::NAME, "Tajikistani somoni");
    assert_eq!(TJS::NUMERIC, 972);
    assert_eq!(TJS::MINOR_UNIT, 2);
    assert_eq!(TJS::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(TJS::THOUSAND_SEPARATOR, ".");
    assert_eq!(TJS::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_tmt_currency() {
    assert_eq!(TMT::CODE, "TMT");
    assert_eq!(TMT::SYMBOL, "¤");
    assert_eq!(TMT::NAME, "Turkmenistan manat");
    assert_eq!(TMT::NUMERIC, 934);
    assert_eq!(TMT::MINOR_UNIT, 2);
    assert_eq!(TMT::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(TMT::THOUSAND_SEPARATOR, ".");
    assert_eq!(TMT::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_tnd_currency() {
    assert_eq!(TND::CODE, "TND");
    assert_eq!(TND::SYMBOL, "د.ت");
    assert_eq!(TND::NAME, "Tunisian dinar");
    assert_eq!(TND::NUMERIC, 788);
    assert_eq!(TND::MINOR_UNIT, 3);
    assert_eq!(TND::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(TND::THOUSAND_SEPARATOR, ".");
    assert_eq!(TND::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_top_currency() {
    assert_eq!(TOP::CODE, "TOP");
    assert_eq!(TOP::SYMBOL, "T$");
    assert_eq!(TOP::NAME, "Tongan paʻanga");
    assert_eq!(TOP::NUMERIC, 776);
    assert_eq!(TOP::MINOR_UNIT, 2);
    assert_eq!(TOP::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(TOP::THOUSAND_SEPARATOR, ".");
    assert_eq!(TOP::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_try_currency() {
    assert_eq!(TRY::CODE, "TRY");
    assert_eq!(TRY::SYMBOL, "₺");
    assert_eq!(TRY::NAME, "Turkish lira");
    assert_eq!(TRY::NUMERIC, 949);
    assert_eq!(TRY::MINOR_UNIT, 2);
    assert_eq!(TRY::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(TRY::THOUSAND_SEPARATOR, ".");
    assert_eq!(TRY::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_ttd_currency() {
    assert_eq!(TTD::CODE, "TTD");
    assert_eq!(TTD::SYMBOL, "$");
    assert_eq!(TTD::NAME, "Trinidad and Tobago dollar");
    assert_eq!(TTD::NUMERIC, 780);
    assert_eq!(TTD::MINOR_UNIT, 2);
    assert_eq!(TTD::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(TTD::THOUSAND_SEPARATOR, ".");
    assert_eq!(TTD::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_twd_currency() {
    assert_eq!(TWD::CODE, "TWD");
    assert_eq!(TWD::SYMBOL, "NT$");
    assert_eq!(TWD::NAME, "New Taiwan dollar");
    assert_eq!(TWD::NUMERIC, 901);
    assert_eq!(TWD::MINOR_UNIT, 2);
    assert_eq!(TWD::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(TWD::THOUSAND_SEPARATOR, ",");
    assert_eq!(TWD::DECIMAL_SEPARATOR, ".");
}

#[test]
fn test_tzs_currency() {
    assert_eq!(TZS::CODE, "TZS");
    assert_eq!(TZS::SYMBOL, "Tsh");
    assert_eq!(TZS::NAME, "Tanzanian shilling");
    assert_eq!(TZS::NUMERIC, 834);
    assert_eq!(TZS::MINOR_UNIT, 2);
    assert_eq!(TZS::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(TZS::THOUSAND_SEPARATOR, ",");
    assert_eq!(TZS::DECIMAL_SEPARATOR, ".");
}

#[test]
fn test_uah_currency() {
    assert_eq!(UAH::CODE, "UAH");
    assert_eq!(UAH::SYMBOL, "₴");
    assert_eq!(UAH::NAME, "Ukrainian hryvnia");
    assert_eq!(UAH::NUMERIC, 980);
    assert_eq!(UAH::MINOR_UNIT, 2);
    assert_eq!(UAH::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(UAH::THOUSAND_SEPARATOR, ".");
    assert_eq!(UAH::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_ugx_currency() {
    assert_eq!(UGX::CODE, "UGX");
    assert_eq!(UGX::SYMBOL, "USh");
    assert_eq!(UGX::NAME, "Ugandan shilling");
    assert_eq!(UGX::NUMERIC, 800);
    assert_eq!(UGX::MINOR_UNIT, 0);
    assert_eq!(UGX::MINOR_UNIT_SYMBOL, "");
    assert_eq!(UGX::THOUSAND_SEPARATOR, ",");
    assert_eq!(UGX::DECIMAL_SEPARATOR, ".");
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

#[test]
fn test_usn_currency() {
    assert_eq!(USN::CODE, "USN");
    assert_eq!(USN::SYMBOL, "$");
    assert_eq!(USN::NAME, "United States dollar (next day)");
    assert_eq!(USN::NUMERIC, 997);
    assert_eq!(USN::MINOR_UNIT, 2);
    assert_eq!(USN::MINOR_UNIT_SYMBOL, "¢");
    assert_eq!(USN::THOUSAND_SEPARATOR, ".");
    assert_eq!(USN::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_uyi_currency() {
    assert_eq!(UYI::CODE, "UYI");
    assert_eq!(UYI::SYMBOL, "¤");
    assert_eq!(UYI::NAME, "Uruguay Peso en Unidades Indexadas (URUIURUI)");
    assert_eq!(UYI::NUMERIC, 940);
    assert_eq!(UYI::MINOR_UNIT, 0);
    assert_eq!(UYI::MINOR_UNIT_SYMBOL, "");
    assert_eq!(UYI::THOUSAND_SEPARATOR, ".");
    assert_eq!(UYI::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_uyu_currency() {
    assert_eq!(UYU::CODE, "UYU");
    assert_eq!(UYU::SYMBOL, "$U");
    assert_eq!(UYU::NAME, "Uruguayan peso");
    assert_eq!(UYU::NUMERIC, 858);
    assert_eq!(UYU::MINOR_UNIT, 2);
    assert_eq!(UYU::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(UYU::THOUSAND_SEPARATOR, ".");
    assert_eq!(UYU::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_uyw_currency() {
    assert_eq!(UYW::CODE, "UYW");
    assert_eq!(UYW::SYMBOL, "¤");
    assert_eq!(UYW::NAME, "Unidad previsional");
    assert_eq!(UYW::NUMERIC, 927);
    assert_eq!(UYW::MINOR_UNIT, 4);
    assert_eq!(UYW::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(UYW::THOUSAND_SEPARATOR, ".");
    assert_eq!(UYW::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_uzs_currency() {
    assert_eq!(UZS::CODE, "UZS");
    assert_eq!(UZS::SYMBOL, "¤");
    assert_eq!(UZS::NAME, "Uzbekistan som");
    assert_eq!(UZS::NUMERIC, 860);
    assert_eq!(UZS::MINOR_UNIT, 2);
    assert_eq!(UZS::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(UZS::THOUSAND_SEPARATOR, ".");
    assert_eq!(UZS::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_ved_currency() {
    assert_eq!(VED::CODE, "VED");
    assert_eq!(VED::SYMBOL, "Bs.");
    assert_eq!(VED::NAME, "Venezuelan bolívar soberano");
    assert_eq!(VED::NUMERIC, 926);
    assert_eq!(VED::MINOR_UNIT, 2);
    assert_eq!(VED::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(VED::THOUSAND_SEPARATOR, ".");
    assert_eq!(VED::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_ves_currency() {
    assert_eq!(VES::CODE, "VES");
    assert_eq!(VES::SYMBOL, "Bs.");
    assert_eq!(VES::NAME, "Venezuelan bolívar soberano");
    assert_eq!(VES::NUMERIC, 928);
    assert_eq!(VES::MINOR_UNIT, 2);
    assert_eq!(VES::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(VES::THOUSAND_SEPARATOR, ".");
    assert_eq!(VES::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_vnd_currency() {
    assert_eq!(VND::CODE, "VND");
    assert_eq!(VND::SYMBOL, "₫");
    assert_eq!(VND::NAME, "Vietnamese đồng");
    assert_eq!(VND::NUMERIC, 704);
    assert_eq!(VND::MINOR_UNIT, 0);
    assert_eq!(VND::MINOR_UNIT_SYMBOL, "");
    assert_eq!(VND::THOUSAND_SEPARATOR, ".");
    assert_eq!(VND::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_vuv_currency() {
    assert_eq!(VUV::CODE, "VUV");
    assert_eq!(VUV::SYMBOL, "VT");
    assert_eq!(VUV::NAME, "Vanuatu vatu");
    assert_eq!(VUV::NUMERIC, 548);
    assert_eq!(VUV::MINOR_UNIT, 0);
    assert_eq!(VUV::MINOR_UNIT_SYMBOL, "");
    assert_eq!(VUV::THOUSAND_SEPARATOR, ".");
    assert_eq!(VUV::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_wst_currency() {
    assert_eq!(WST::CODE, "WST");
    assert_eq!(WST::SYMBOL, "WS$");
    assert_eq!(WST::NAME, "Samoan tala");
    assert_eq!(WST::NUMERIC, 882);
    assert_eq!(WST::MINOR_UNIT, 2);
    assert_eq!(WST::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(WST::THOUSAND_SEPARATOR, ".");
    assert_eq!(WST::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_xaf_currency() {
    assert_eq!(XAF::CODE, "XAF");
    assert_eq!(XAF::SYMBOL, "FCFA");
    assert_eq!(XAF::NAME, "CFA franc BEAC");
    assert_eq!(XAF::NUMERIC, 950);
    assert_eq!(XAF::MINOR_UNIT, 0);
    assert_eq!(XAF::MINOR_UNIT_SYMBOL, "");
    assert_eq!(XAF::THOUSAND_SEPARATOR, ".");
    assert_eq!(XAF::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_xag_currency() {
    assert_eq!(XAG::CODE, "XAG");
    assert_eq!(XAG::SYMBOL, "¤");
    assert_eq!(XAG::NAME, "Silver (one troy ounce)");
    assert_eq!(XAG::NUMERIC, 961);
    assert_eq!(XAG::MINOR_UNIT, 0);
    assert_eq!(XAG::MINOR_UNIT_SYMBOL, "");
    assert_eq!(XAG::THOUSAND_SEPARATOR, ".");
    assert_eq!(XAG::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_xau_currency() {
    assert_eq!(XAU::CODE, "XAU");
    assert_eq!(XAU::SYMBOL, "¤");
    assert_eq!(XAU::NAME, "Gold (one troy ounce)");
    assert_eq!(XAU::NUMERIC, 959);
    assert_eq!(XAU::MINOR_UNIT, 0);
    assert_eq!(XAU::MINOR_UNIT_SYMBOL, "");
    assert_eq!(XAU::THOUSAND_SEPARATOR, ".");
    assert_eq!(XAU::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_xba_currency() {
    assert_eq!(XBA::CODE, "XBA");
    assert_eq!(XBA::SYMBOL, "¤");
    assert_eq!(XBA::NAME, "European Composite Unit (EURCO)");
    assert_eq!(XBA::NUMERIC, 955);
    assert_eq!(XBA::MINOR_UNIT, 0);
    assert_eq!(XBA::MINOR_UNIT_SYMBOL, "");
    assert_eq!(XBA::THOUSAND_SEPARATOR, ".");
    assert_eq!(XBA::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_xbb_currency() {
    assert_eq!(XBB::CODE, "XBB");
    assert_eq!(XBB::SYMBOL, "¤");
    assert_eq!(XBB::NAME, "European Monetary Unit (E.M.U.-6)");
    assert_eq!(XBB::NUMERIC, 956);
    assert_eq!(XBB::MINOR_UNIT, 0);
    assert_eq!(XBB::MINOR_UNIT_SYMBOL, "");
    assert_eq!(XBB::THOUSAND_SEPARATOR, ".");
    assert_eq!(XBB::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_xbc_currency() {
    assert_eq!(XBC::CODE, "XBC");
    assert_eq!(XBC::SYMBOL, "¤");
    assert_eq!(XBC::NAME, "European Unit of Account 9 (E.U.A.-9)");
    assert_eq!(XBC::NUMERIC, 957);
    assert_eq!(XBC::MINOR_UNIT, 0);
    assert_eq!(XBC::MINOR_UNIT_SYMBOL, "");
    assert_eq!(XBC::THOUSAND_SEPARATOR, ".");
    assert_eq!(XBC::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_xbd_currency() {
    assert_eq!(XBD::CODE, "XBD");
    assert_eq!(XBD::SYMBOL, "¤");
    assert_eq!(XBD::NAME, "European Unit of Account 17 (E.U.A.-17)");
    assert_eq!(XBD::NUMERIC, 958);
    assert_eq!(XBD::MINOR_UNIT, 0);
    assert_eq!(XBD::MINOR_UNIT_SYMBOL, "");
    assert_eq!(XBD::THOUSAND_SEPARATOR, ".");
    assert_eq!(XBD::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_xcd_currency() {
    assert_eq!(XCD::CODE, "XCD");
    assert_eq!(XCD::SYMBOL, "$");
    assert_eq!(XCD::NAME, "East Caribbean dollar");
    assert_eq!(XCD::NUMERIC, 951);
    assert_eq!(XCD::MINOR_UNIT, 2);
    assert_eq!(XCD::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(XCD::THOUSAND_SEPARATOR, ".");
    assert_eq!(XCD::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_xcg_currency() {
    assert_eq!(XCG::CODE, "XCG");
    assert_eq!(XCG::SYMBOL, "ƒ");
    assert_eq!(XCG::NAME, "Caribean guilder");
    assert_eq!(XCG::NUMERIC, 532);
    assert_eq!(XCG::MINOR_UNIT, 2);
    assert_eq!(XCG::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(XCG::THOUSAND_SEPARATOR, ".");
    assert_eq!(XCG::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_xdr_currency() {
    assert_eq!(XDR::CODE, "XDR");
    assert_eq!(XDR::SYMBOL, "SDR");
    assert_eq!(XDR::NAME, "Special drawing rights");
    assert_eq!(XDR::NUMERIC, 960);
    assert_eq!(XDR::MINOR_UNIT, 0);
    assert_eq!(XDR::MINOR_UNIT_SYMBOL, "");
    assert_eq!(XDR::THOUSAND_SEPARATOR, ".");
    assert_eq!(XDR::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_xof_currency() {
    assert_eq!(XOF::CODE, "XOF");
    assert_eq!(XOF::SYMBOL, "CFA");
    assert_eq!(XOF::NAME, "CFA franc BCEAO");
    assert_eq!(XOF::NUMERIC, 952);
    assert_eq!(XOF::MINOR_UNIT, 0);
    assert_eq!(XOF::MINOR_UNIT_SYMBOL, "");
    assert_eq!(XOF::THOUSAND_SEPARATOR, ".");
    assert_eq!(XOF::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_xpd_currency() {
    assert_eq!(XPD::CODE, "XPD");
    assert_eq!(XPD::SYMBOL, "¤");
    assert_eq!(XPD::NAME, "Palladium (one troy ounce)");
    assert_eq!(XPD::NUMERIC, 964);
    assert_eq!(XPD::MINOR_UNIT, 0);
    assert_eq!(XPD::MINOR_UNIT_SYMBOL, "");
    assert_eq!(XPD::THOUSAND_SEPARATOR, ".");
    assert_eq!(XPD::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_xpf_currency() {
    assert_eq!(XPF::CODE, "XPF");
    assert_eq!(XPF::SYMBOL, "₣");
    assert_eq!(XPF::NAME, "CFP franc (franc Pacifique)");
    assert_eq!(XPF::NUMERIC, 953);
    assert_eq!(XPF::MINOR_UNIT, 0);
    assert_eq!(XPF::MINOR_UNIT_SYMBOL, "");
    assert_eq!(XPF::THOUSAND_SEPARATOR, ".");
    assert_eq!(XPF::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_xpt_currency() {
    assert_eq!(XPT::CODE, "XPT");
    assert_eq!(XPT::SYMBOL, "¤");
    assert_eq!(XPT::NAME, "Platinum (one troy ounce)");
    assert_eq!(XPT::NUMERIC, 962);
    assert_eq!(XPT::MINOR_UNIT, 0);
    assert_eq!(XPT::MINOR_UNIT_SYMBOL, "");
    assert_eq!(XPT::THOUSAND_SEPARATOR, ".");
    assert_eq!(XPT::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_xsu_currency() {
    assert_eq!(XSU::CODE, "XSU");
    assert_eq!(XSU::SYMBOL, "¤");
    assert_eq!(XSU::NAME, "SUCRE");
    assert_eq!(XSU::NUMERIC, 994);
    assert_eq!(XSU::MINOR_UNIT, 0);
    assert_eq!(XSU::MINOR_UNIT_SYMBOL, "");
    assert_eq!(XSU::THOUSAND_SEPARATOR, ".");
    assert_eq!(XSU::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_xts_currency() {
    assert_eq!(XTS::CODE, "XTS");
    assert_eq!(XTS::SYMBOL, "¤");
    assert_eq!(XTS::NAME, "Code reserved for testing");
    assert_eq!(XTS::NUMERIC, 963);
    assert_eq!(XTS::MINOR_UNIT, 0);
    assert_eq!(XTS::MINOR_UNIT_SYMBOL, "");
    assert_eq!(XTS::THOUSAND_SEPARATOR, ".");
    assert_eq!(XTS::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_xua_currency() {
    assert_eq!(XUA::CODE, "XUA");
    assert_eq!(XUA::SYMBOL, "¤");
    assert_eq!(XUA::NAME, "ADB Unit of Account");
    assert_eq!(XUA::NUMERIC, 965);
    assert_eq!(XUA::MINOR_UNIT, 0);
    assert_eq!(XUA::MINOR_UNIT_SYMBOL, "");
    assert_eq!(XUA::THOUSAND_SEPARATOR, ".");
    assert_eq!(XUA::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_xxx_currency() {
    assert_eq!(XXX::CODE, "XXX");
    assert_eq!(XXX::SYMBOL, "¤");
    assert_eq!(XXX::NAME, "No currency");
    assert_eq!(XXX::NUMERIC, 999);
    assert_eq!(XXX::MINOR_UNIT, 0);
    assert_eq!(XXX::MINOR_UNIT_SYMBOL, "");
    assert_eq!(XXX::THOUSAND_SEPARATOR, ".");
    assert_eq!(XXX::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_yer_currency() {
    assert_eq!(YER::CODE, "YER");
    assert_eq!(YER::SYMBOL, "ر.ي");
    assert_eq!(YER::NAME, "Yemeni rial");
    assert_eq!(YER::NUMERIC, 886);
    assert_eq!(YER::MINOR_UNIT, 2);
    assert_eq!(YER::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(YER::THOUSAND_SEPARATOR, ".");
    assert_eq!(YER::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_zar_currency() {
    assert_eq!(ZAR::CODE, "ZAR");
    assert_eq!(ZAR::SYMBOL, "R");
    assert_eq!(ZAR::NAME, "South African rand");
    assert_eq!(ZAR::NUMERIC, 710);
    assert_eq!(ZAR::MINOR_UNIT, 2);
    assert_eq!(ZAR::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(ZAR::THOUSAND_SEPARATOR, ",");
    assert_eq!(ZAR::DECIMAL_SEPARATOR, ".");
}

#[test]
fn test_zmw_currency() {
    assert_eq!(ZMW::CODE, "ZMW");
    assert_eq!(ZMW::SYMBOL, "K");
    assert_eq!(ZMW::NAME, "Zambian kwacha");
    assert_eq!(ZMW::NUMERIC, 967);
    assert_eq!(ZMW::MINOR_UNIT, 2);
    assert_eq!(ZMW::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(ZMW::THOUSAND_SEPARATOR, ",");
    assert_eq!(ZMW::DECIMAL_SEPARATOR, ".");
}

#[test]
fn test_zwg_currency() {
    assert_eq!(ZWG::CODE, "ZWG");
    assert_eq!(ZWG::SYMBOL, "¤");
    assert_eq!(ZWG::NAME, "Zimbabwe Gold");
    assert_eq!(ZWG::NUMERIC, 924);
    assert_eq!(ZWG::MINOR_UNIT, 2);
    assert_eq!(ZWG::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(ZWG::THOUSAND_SEPARATOR, ".");
    assert_eq!(ZWG::DECIMAL_SEPARATOR, ",");
}

#[test]
fn test_zwl_currency() {
    assert_eq!(ZWL::CODE, "ZWL");
    assert_eq!(ZWL::SYMBOL, "¤");
    assert_eq!(ZWL::NAME, "Zimbabwean dollar");
    assert_eq!(ZWL::NUMERIC, 932);
    assert_eq!(ZWL::MINOR_UNIT, 2);
    assert_eq!(ZWL::MINOR_UNIT_SYMBOL, "minor");
    assert_eq!(ZWL::THOUSAND_SEPARATOR, ".");
    assert_eq!(ZWL::DECIMAL_SEPARATOR, ",");
}
