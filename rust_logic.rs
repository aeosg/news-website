#[no_mangle]
pub extern "C" fn compute_urgency(time_ago_mins: i32, importance: i32) -> i32 {
    let mut urgency = importance * 10;
    if time_ago_mins < 60 {
        urgency += 50;
    } else if time_ago_mins < 1440 {
        urgency += 20;
    }

    if urgency > 100 {
        100
    } else if urgency < 0 {
        0
    } else {
        urgency
    }
}
