//! This module provides function for day count fraction calculation.

use chrono::{Datelike, Duration, NaiveDate, Weekday};
use rust_decimal::{prelude::*, Decimal};
use rust_decimal_macros::dec;

const NON_LEAP: Decimal = dec!(365);
const LEAP: Decimal = dec!(366);
const THREE_SIXTY: Decimal = dec!(360);


/// Returns an `ACT/360` day count fraction for the dates provided.
pub fn act_360(start: NaiveDate, end: NaiveDate) -> Option<Decimal> {
    if start > end {
        return None;
    }

    Some(Decimal::new((end - start).num_days(), 0) / THREE_SIXTY)
}

/// Returns an `ACT/365 (Fixed)` day count fraction for the dates provided.
pub fn act_365f(start: NaiveDate, end: NaiveDate) -> Option<Decimal> {
    if start > end {
        return None;
    }

    Some(Decimal::new((end - start).num_days(), 0) / NON_LEAP)
}

/// Returns an `ACT/ACT (ISDA)` day count fraction
pub fn act_act_isda(start: NaiveDate, end: NaiveDate) -> Option<Decimal> {
    if start > end {
        return None;
    }
    let mut year = start.year();

    if year == end.year() {
        if is_leap(year) {
            return Some(Decimal::new((end - start).num_days(), 0) / LEAP);
        }
        return Some(Decimal::new((end - start).num_days(), 0) / NON_LEAP);        
    }

    let mut dcf = if is_leap(year) {
        Decimal::new(366 - i64::from(start.ordinal()), 0) / LEAP
    } else {
        Decimal::new(365 - i64::from(start.ordinal()), 0) / NON_LEAP
    };
    

    year += 1;  


    while year < end.year() {
        dcf += Decimal::new(1, 0);
        year += 1;
    }

    if is_leap(year) {
        dcf += Decimal::new(i64::from(end.ordinal()), 0) / LEAP;
    } else {
        dcf += Decimal::new(i64::from(end.ordinal()), 0) / NON_LEAP;
    };

    Some(dcf)
}

/// Calculates `ACT/ACT (ICMA)` day count fraction for the given dates.
pub fn act_act_isma(start: NaiveDate, end: NaiveDate) -> Option<Decimal> {
    if start > end {
        return None;
    }
}

/// Returns a `30/360` day count fraction for the given dates.
pub fn d30_360(start: NaiveDate, end: NaiveDate) -> Option<Decimal> {
    if start > end {
        return None;
    }

    let start_day = if start.day() == 31 {
        30
    } else {
        start.day()
    };

    let end_day = if end.day() == 31 && start_day == 30 {
        30
    } else {
        end.day()
    };

    let years = i64::from(end.year() - start.year());
    let months = i64::from(end.month() - start.month());
    let days = i64::from(end_day - start_day);
    let day_count = years * 360 + months * 30 + days;

    Some(Decimal::new(day_count, 0) / Decimal::new(360, 0))

}

/// Checks whether a year is a leap year.
fn is_leap(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

