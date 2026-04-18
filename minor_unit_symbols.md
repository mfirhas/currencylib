# Minor Unit Symbols Research

## Background

The `iso_currency` crate used by this library only provides `subunit_symbol` for 14 currencies. For all other currencies with a non-zero minor unit (exponent), the build script defaults `MINOR_UNIT_SYMBOL` to `"minor"`. This document catalogues what the actual minor unit symbols should be for those **138 currencies**.

### Currencies already handled by upstream

| Code | Currency | Symbol |
|------|----------|--------|
| ALL | Albanian lek | q |
| AUD | Australian dollar | c |
| BTN | Bhutanese ngultrum | Ch. |
| CAD | Canadian dollar | ¢ |
| CZK | Czech koruna | h |
| EGP | Egyptian pound | pt |
| GBP | British pound | p |
| HRK | Croatian kuna | lp |
| MXN | Mexican peso | ¢ |
| NAD | Namibian dollar | NA |
| NZD | New Zealand dollar | c |
| PLN | Polish zloty | gr |
| USD | US dollar | ¢ |
| USN | US dollar (next day) | ¢ |

---

## Currencies needing minor unit symbols

### Cent family (¢) — subunit is "cent", "centavo", "céntimo", or "centésimo"

| Code | Currency | Subunit Name | Suggested Symbol |
|------|----------|--------------|------------------|
| ARS | Argentine peso | centavo | ¢ |
| AWG | Aruban florin | cent | ¢ |
| BBD | Barbados dollar | cent | ¢ |
| BMD | Bermudian dollar | cent | ¢ |
| BOB | Boliviano | centavo | ¢ |
| BRL | Brazilian real | centavo | ¢ |
| BSD | Bahamian dollar | cent | ¢ |
| BZD | Belize dollar | cent | ¢ |
| COP | Colombian peso | centavo | ¢ |
| CRC | Costa Rican colón | céntimo | ¢ |
| CUC | Cuban convertible peso | centavo | ¢ |
| CUP | Cuban peso | centavo | ¢ |
| CVE | Cape Verdean escudo | centavo | ¢ |
| DOP | Dominican peso | centavo | ¢ |
| ERN | Eritrean nakfa | cent | ¢ |
| EUR | Euro | cent | c |
| FJD | Fiji dollar | cent | ¢ |
| GTQ | Guatemalan quetzal | centavo | ¢ |
| GYD | Guyanese dollar | cent | ¢ |
| HKD | Hong Kong dollar | cent | ¢ |
| HNL | Honduran lempira | centavo | ¢ |
| JMD | Jamaican dollar | cent | ¢ |
| KES | Kenyan shilling | cent | ¢ |
| KYD | Cayman Islands dollar | cent | ¢ |
| LKR | Sri Lankan rupee | cent | ¢ |
| LRD | Liberian dollar | cent | ¢ |
| MUR | Mauritian rupee | cent | ¢ |
| MZN | Mozambican metical | centavo | ¢ |
| NIO | Nicaraguan córdoba | centavo | ¢ |
| PAB | Panamanian balboa | centésimo | ¢ |
| PEN | Peruvian sol | céntimo | ¢ |
| PHP | Philippine peso | sentimo/centavo | ¢ |
| SBD | Solomon Islands dollar | cent | ¢ |
| SCR | Seychelles rupee | cent | ¢ |
| SGD | Singapore dollar | cent | ¢ |
| SLE | Sierra Leonean leone | cent | ¢ |
| SLL | Sierra Leonean leone (old) | cent | ¢ |
| SOS | Somali shilling | senti | ¢ |
| SRD | Surinamese dollar | cent | ¢ |
| STN | São Tomé and Príncipe dobra | cêntimo | ¢ |
| SVC | Salvadoran colón | centavo | ¢ |
| SZL | Swazi lilangeni | cent | ¢ |
| TTD | Trinidad and Tobago dollar | cent | ¢ |
| TWD | New Taiwan dollar | cent/分 | ¢ |
| TZS | Tanzanian shilling | senti | ¢ |
| UYU | Uruguayan peso | centésimo | ¢ |
| VED | Venezuelan bolívar digital | céntimo | ¢ |
| VES | Venezuelan bolívar soberano | céntimo | ¢ |
| XCD | East Caribbean dollar | cent | ¢ |
| ZAR | South African rand | cent | c |
| ZMW | Zambian kwacha | ngwee | ¢ |
| ZWG | Zimbabwe Gold | cent | ¢ |
| ZWL | Zimbabwean dollar | cent | ¢ |

### Penny / Pence / Paisa family (p)

| Code | Currency | Subunit Name | Suggested Symbol |
|------|----------|--------------|------------------|
| FKP | Falkland Islands pound | penny | p |
| GIP | Gibraltar pound | penny | p |
| SHP | Saint Helena pound | penny | p |
| INR | Indian rupee | paisa | p |
| NPR | Nepalese rupee | paisa | p |
| PKR | Pakistani rupee | paisa | p |
| BDT | Bangladeshi taka | poisha | p |

### Centime family (c)

| Code | Currency | Subunit Name | Suggested Symbol |
|------|----------|--------------|------------------|
| CDF | Congolese franc | centime | c |
| CHF | Swiss franc | Rappen/centime | c |
| DZD | Algerian dinar | centime/santim | c |
| HTG | Haitian gourde | centime | c |
| MAD | Moroccan dirham | centime/santim | c |

### Fils / dirham family

| Code | Currency | Subunit Name | Suggested Symbol |
|------|----------|--------------|------------------|
| AED | UAE dirham | fils | فلس |
| BHD | Bahraini dinar | fils | فلس |
| IQD | Iraqi dinar | fils | فلس |
| JOD | Jordanian dinar | fils | فلس |
| KWD | Kuwaiti dinar | fils | فلس |
| YER | Yemeni rial | fils | فلس |

### Øre family

| Code | Currency | Subunit Name | Suggested Symbol |
|------|----------|--------------|------------------|
| DKK | Danish krone | øre | øre |
| NOK | Norwegian krone | øre | øre |
| SEK | Swedish krona | öre | öre |

### Other currencies with specific subunit symbols

| Code | Currency | Subunit Name | Suggested Symbol |
|------|----------|--------------|------------------|
| AFN | Afghan afghani | pul | پول |
| AMD | Armenian dram | luma | լ |
| AOA | Angolan kwanza | cêntimo | ¢ |
| AZN | Azerbaijani manat | qəpik | q |
| BAM | Bosnia and Herzegovina convertible mark | fening | pf |
| BGN | Bulgarian lev | stotinka | ст. |
| BND | Brunei dollar | sen | sen |
| BWP | Botswana pula | thebe | t |
| BYN | Belarusian ruble | kopek | коп. |
| CNY | Chinese yuan | fen (分) | 分 |
| ETB | Ethiopian birr | santim | c |
| GEL | Georgian lari | tetri | ₮ |
| GHS | Ghanaian cedi | pesewa | Gp |
| GMD | Gambian dalasi | butut | b |
| HUF | Hungarian forint | fillér | f |
| IDR | Indonesian rupiah | sen | sen |
| ILS | Israeli new shekel | agora | ag. |
| IRR | Iranian rial | dinar | — |
| KGS | Kyrgyzstani som | tyiyn | тыйын |
| KHR | Cambodian riel | sen | sen |
| KPW | North Korean won | chon | 전 |
| KZT | Kazakhstani tenge | tïın | тиын |
| LAK | Lao kip | att | att |
| LBP | Lebanese pound | piastre | — |
| LSL | Lesotho loti | sente | s |
| LYD | Libyan dinar | dirham | د |
| MDL | Moldovan leu | ban | b |
| MGA | Malagasy ariary | iraimbilanja | Fy |
| MKD | Macedonian denar | deni | ден |
| MMK | Myanmar kyat | pya | pya |
| MNT | Mongolian tögrög | möngö | ₮ |
| MOP | Macanese pataca | avo | avo |
| MRU | Mauritanian ouguiya | khoums | خ |
| MVR | Maldivian rufiyaa | laari | ލ |
| MWK | Malawian kwacha | tambala | t |
| MYR | Malaysian ringgit | sen | sen |
| NGN | Nigerian naira | kobo | k |
| OMR | Omani rial | baisa | ب.ع |
| PGK | Papua New Guinean kina | toea | t |
| QAR | Qatari riyal | dirham | د.ق |
| RON | Romanian leu | ban | b |
| RSD | Serbian dinar | para | пар. |
| RUB | Russian ruble | kopek | коп. |
| SAR | Saudi riyal | halala | هللة |
| SDG | Sudanese pound | piastre/qirsh | قرش |
| SSP | South Sudanese pound | piaster | pt |
| SYP | Syrian pound | piastre | قرش |
| THB | Thai baht | satang | สต. |
| TJS | Tajikistani somoni | diram | д |
| TMT | Turkmenistan manat | tenge | t |
| TND | Tunisian dinar | millime | m |
| TOP | Tongan paʻanga | seniti | s |
| TRY | Turkish lira | kuruş | kr |
| UAH | Ukrainian hryvnia | kopiyka | коп. |
| UZS | Uzbekistan som | tiyin | тийин |
| WST | Samoan tala | sene | s |

### Special / Fund currencies (minor unit symbol not practically meaningful)

| Code | Currency | Notes |
|------|----------|-------|
| BOV | Bolivian Mvdol | Fund currency |
| CHE | WIR Euro | Fund currency |
| CHW | WIR Franc | Fund currency |
| CLF | Unidad de Fomento | Indexing unit (exponent 4) |
| COU | Unidad de Valor Real | Fund currency |
| MXV | Mexican Unidad de Inversion | Fund/indexing unit |
| UYW | Unidad previsional | Fund currency (exponent 4) |
| XCG | Caribbean guilder | New currency, ¢ likely |

---

## Summary

| Category | Count | Confidence |
|----------|-------|------------|
| Cent family (¢ or c) | ~53 | High |
| Penny/Paisa family (p) | 7 | High |
| Centime family (c) | 5 | High |
| Fils family (فلس) | 6 | High |
| Øre family | 3 | High |
| Other specific symbols | ~56 | Medium — symbols vary by source |
| Special/fund currencies | 8 | N/A — no practical symbol needed |
| **Total** | **138** | |

> **Note:** These findings are based on widely known currency conventions. Some subunit symbols (especially for currencies using non-Latin scripts) may have regional variations. The "cent family" and "penny/paisa family" are the highest-confidence recommendations.
