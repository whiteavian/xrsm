extern crate chrono;
extern crate time;

use chrono::*;
use time::Duration;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let start_date = Utc.ymd(2011, 4, 25).and_hms(0,0,0);
    start_date + Duration::seconds(i64::from(10).pow(9))
}
