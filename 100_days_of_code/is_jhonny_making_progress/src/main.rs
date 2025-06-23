fn progress_days(days: Vec<u16>) -> u16 {
    let mut progress = 0;
    for i in 0..days.len() - 1 {
        if days[i] < days[i + 1] {
            progress += 1;
        }
    }
    progress
}

fn day_text(num: u16) -> String {
    if num == 1 {
        return "day".to_string();
    }
    return "days".to_string();
}

fn main() {
    let progress = progress_days(vec![6, 5, 4, 3, 2, 9]);

    println!("Jhonny progressed for {} {}", progress, day_text(progress));
}
