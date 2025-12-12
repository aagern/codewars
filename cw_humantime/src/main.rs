use std::time::Duration;

fn make_readable(seconds: u32) -> String {
    let duration = Duration::from_secs(seconds as u64);

    let hours = duration.as_secs() / 3600;
    let minutes = (duration.as_secs() % 3600) / 60;
    let seconds = duration.as_secs() % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

fn main() {
    println!("{}", make_readable(86399));
}
