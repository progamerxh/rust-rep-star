use std::str::FromStr;

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
