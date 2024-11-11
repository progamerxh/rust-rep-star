use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_urlencoded;

#[derive(Debug, PartialEq)]
pub enum TimeDuration {
    LastDay,
    LastWeek,
    LastMonth,
    LastYear,
}

impl FromStr for TimeDuration {
    type Err = ();

    fn from_str(input: &str) -> Result<TimeDuration, Self::Err> {
        match input {
            "day" => Ok(TimeDuration::LastDay),
            "week" => Ok(TimeDuration::LastWeek),
            "month" => Ok(TimeDuration::LastMonth),
            "year" => Ok(TimeDuration::LastYear),
            _ => Err(()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TestimonialQueries {
    pub q: Option<String>,
}

impl TestimonialQueries {
    pub fn to_query_string(&self) -> String {
        serde_urlencoded::to_string(self).unwrap_or_default()
    }
}
