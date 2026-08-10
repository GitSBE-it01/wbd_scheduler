use time::{OffsetDateTime, Weekday};

#[tokio::main]
async fn main() {
    // 1. Get current date & time (UTC)
    let now = OffsetDateTime::now_local()
        .unwrap_or_else(|_| OffsetDateTime::now_utc())
        .date();

    // 2. Extract the weekday
    let current_day = now.weekday();

    // 3. Print the day name directly (e.g., "Monday")
    println!("Today is: {}", current_day);

    // 4. Test your special condition
    if matches!(current_day, Weekday::Sunday | Weekday::Monday) {
        println!("Special condition active for {}!", current_day);
    } else {
        println!("Regular schedule for {}.", current_day);
    }
}
