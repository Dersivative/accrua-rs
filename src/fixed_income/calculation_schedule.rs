use chrono::NaiveDate;

pub struct Schedule {
    first: Option<NaiveDate>,
    first_regular: NaiveDate,
    last: Option<NaiveDate>,
    last_regular: NaiveDate
    
}