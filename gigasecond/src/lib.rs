use chrono::{DateTime, Utc};
use chrono::*;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
 
    let sec=Duration::seconds(1000000000);
    start+sec
}
