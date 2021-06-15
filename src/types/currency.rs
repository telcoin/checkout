use bigdecimal::{BigDecimal, ToPrimitive};
use serde::{Deserialize, Serialize};

/// The monetary value that is scaled to an integer based on its currency.
///
/// See [Calculating the value](https://docs.checkout.com/resources/calculating-the-value)
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Amount(u64);

/// These are the major currencies supported
///
/// See [Currency Codes](https://docs.checkout.com/resources/codes/currency-codes)
#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum Currency {
    AED,
    AFN,
    ALL,
    AMD,
    ANG,
    AOA,
    ARS,
    AUD,
    AWG,
    AZN,
    BAM,
    BBD,
    BDT,
    BGN,
    BHD,
    BIF,
    BMD,
    BND,
    BOB,
    BRL,
    BSD,
    BTN,
    BWP,
    BYN,
    BZD,
    CAD,
    CDF,
    CHF,
    CLF,
    CLP,
    CNY,
    COP,
    CRC,
    CVE,
    CZK,
    DJF,
    DKK,
    DOP,
    DZD,
    EEK,
    EGP,
    ERN,
    ETB,
    EUR,
    FJD,
    FKP,
    GBP,
    GEL,
    GHS,
    GIP,
    GMD,
    GNF,
    GTQ,
    GYD,
    HKD,
    HNL,
    HRK,
    HTG,
    HUF,
    IDR,
    ILS,
    INR,
    IQD,
    IRR,
    ISK,
    JMD,
    JOD,
    JPY,
    KES,
    KGS,
    KHR,
    KMF,
    KPW,
    KRW,
    KWD,
    KYD,
    KZT,
    LAK,
    LBP,
    LKR,
    LRD,
    LSL,
    LTL,
    LVL,
    LYD,
    MAD,
    MDL,
    MGA,
    MKD,
    MMK,
    MNT,
    MOP,
    MRO,
    MUR,
    MVR,
    MWK,
    MXN,
    MYR,
    MZN,
    NAD,
    NGN,
    NIO,
    NOK,
    NPR,
    NZD,
    OMR,
    PAB,
    PEN,
    PGK,
    PHP,
    PKR,
    PLN,
    PYG,
    QAR,
    RON,
    RSD,
    RUB,
    RWF,
    SAR,
    SBD,
    SCR,
    SDG,
    SEK,
    SGD,
    SHP,
    SLL,
    SOS,
    SRD,
    STD,
    SVC,
    SYP,
    SZL,
    THB,
    TJS,
    TMT,
    TND,
    TOP,
    TRY,
    TTD,
    TWD,
    TZS,
    UAH,
    UGX,
    USD,
    UYU,
    UZS,
    VES,
    VND,
    VUV,
    WST,
    XAF,
    XCD,
    XOF,
    XPF,
    YER,
    ZAR,
    ZMW,
    ZWL,
}

impl Amount {
    /// Creates the amount from the raw value and currency. The currency is
    /// required since the value is encoded as a scaled integer, which is
    /// different depending on the currency.
    pub fn into(self, currency: Currency) -> BigDecimal {
        match currency {
            Currency::BIF
            | Currency::CLF
            | Currency::DJF
            | Currency::GNF
            | Currency::ISK
            | Currency::JPY
            | Currency::KMF
            | Currency::KRW
            | Currency::PYG
            | Currency::RWF
            | Currency::UGX
            | Currency::VND
            | Currency::VUV
            | Currency::XAF
            | Currency::XOF
            | Currency::XPF => {
                // For the following currencies, the value is the same as the
                // full charge amount. For example, amount = 100 is equal to
                // 100 Japanese Yen.
                BigDecimal::from(self.0)
            }
            Currency::BHD
            | Currency::IQD
            | Currency::JOD
            | Currency::KWD
            | Currency::LYD
            | Currency::OMR
            | Currency::TND => {
                // With the following currencies, divide the value by 1000 to
                // work out the value amount. For example, value = 1000 is the
                // same as 1 Bahraini Dinar.
                BigDecimal::from(self.0) / BigDecimal::from(1000)
            }
            _ => {
                // For all other currencies, divide the value by 100 to
                // calculate the charge amount. For example, value = 100 is
                // equivalent to 1 US Dollar.
                BigDecimal::from(self.0) / BigDecimal::from(100)
            }
        }
    }

    /// Creates the amount from the raw value and currency. The currency is
    /// required since the value is encoded as a scaled integer, which is
    /// different depending on the currency.
    pub fn from(currency: Currency, amount: BigDecimal) -> Amount {
        match currency {
            Currency::BIF
            | Currency::CLF
            | Currency::DJF
            | Currency::GNF
            | Currency::ISK
            | Currency::JPY
            | Currency::KMF
            | Currency::KRW
            | Currency::PYG
            | Currency::RWF
            | Currency::UGX
            | Currency::VND
            | Currency::VUV
            | Currency::XAF
            | Currency::XOF
            | Currency::XPF => {
                // For the following currencies, the value is the same as the
                // full charge amount. For example, amount = 100 is equal to
                // 100 Japanese Yen.
                Amount(amount.to_u64().unwrap())
            }
            Currency::BHD
            | Currency::IQD
            | Currency::JOD
            | Currency::KWD
            | Currency::LYD
            | Currency::OMR
            | Currency::TND => {
                // With the following currencies, divide the value by 1000 to
                // work out the value amount. For example, value = 1000 is the
                // same as 1 Bahraini Dinar.
                Amount((amount * BigDecimal::from(1000)).to_u64().unwrap())
            }
            _ => {
                // For all other currencies, divide the value by 100 to
                // calculate the charge amount. For example, value = 100 is
                // equivalent to 1 US Dollar.
                Amount((amount * BigDecimal::from(100)).to_u64().unwrap())
            }
        }
    }
}
