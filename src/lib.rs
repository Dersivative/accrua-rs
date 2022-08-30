#![deny(missing_docs)]
#![warn(rust_2018_idioms)]
#![warn(clippy::pedantic)]

//! accrua-rs is a acrual calculation library, providing tool used for calculation
//! of the fixed income (including OTC derivatives) coupons and prededicting their
//! settlement dates based on the bank and currency holidays, as well as the
//! conventions used.

pub mod calendar;
pub mod fixed_income;

/// A `BusinessDayConvetion` represents the method of date rolling in case
/// it falls on a non-business day.
#[derive(Debug)]
#[non_exhaustive]
pub enum BusinessDayConvetion {
    /// The adjusted date will be the first business day following the unadjusted date.
    Following,
    /// The adjusted date will be the first business day following the unadjusted date,
    /// unless it falls in the next calendar month - then the first preceding business day
    /// is the adjusted date.
    ModifiedFollowiing,
    /// The adjusted date will be the first business day preceding the unadjusted date.
    Preceding,
    /// The adjusted date will be the first business day preceding the unadjusted date,
    /// unless it falls in the previous calendar month - then the first following business day
    /// is the adjusted date.
    ModifiedPreceding,
    /// No date adjustment is made.
    NoAdjustment,
}
