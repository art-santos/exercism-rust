use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    use time::{Duration};

    let duration = Duration::seconds(1000000000);

    let new_date = start.checked_add(duration);

    new_date.unwrap()
}
