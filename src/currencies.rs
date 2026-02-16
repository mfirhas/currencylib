/// Trait that defines common currency metadata
pub trait Currency {
    /// ISO 4217 currency code (e.g., "USD", "EUR")
    const CODE: &'static str;
    
    /// Currency symbol (e.g., "$", "€")
    const SYMBOL: &'static str;
    
    /// Full currency name (e.g., "US Dollar", "Euro")
    const NAME: &'static str;
    
    /// Number of decimal places (minor units)
    const DECIMAL_PLACES: u32;
    
    /// Name of the minor unit (e.g., "cent", "penny")
    const MINOR_UNIT_NAME: &'static str;
}

// Include the generated currency structs and implementations
include!(concat!(env!("OUT_DIR"), "/currencies.rs"));
