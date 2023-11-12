use chrono::{Duration, Local};
use chrono_english::{parse_date_string, Dialect};
use std::env;
use std::process::exit;

const DATE_ARG: &str = "DATE|now|today|tomorrow|...";

fn print_help() {
    let cmd = env::args().next().unwrap();
    println!(
        "Usage: {cmd} [{DATE_ARG}] until {DATE_ARG}

Examples:
    {cmd} until 25 dec
    {cmd} until next 25 dec
    {cmd} 1 January 1970 until now
    {cmd} until 23 may 2030
"
    );
}

fn error(msg: &str) -> ! {
    println!("{msg}");
    print_help();
    exit(1);
}

fn main() {
    let cmd_line = env::args().skip(1).collect::<Vec<_>>().join(" ");

    if matches!(cmd_line.as_str(), "--help" | "-h" | "help") {
        print_help();
        exit(0);
    }

    let now = Local::now();

    let parse_date =
        |s, error_msg| parse_date_string(s, now, Dialect::Uk).unwrap_or_else(|_| error(error_msg));

    let (start, end) = cmd_line
        .split_once("until ")
        .filter(|(start, _)| start.is_empty() || start.ends_with(|c: char| c.is_whitespace()))
        .unwrap_or_else(|| error("Invalid syntax"));

    let start_date = if start.is_empty() { now } else { parse_date(start, "Invalid start date") };
    let end_date = parse_date(end, "Invalid end date");

    let diff = end_date - start_date;
    let neg = if diff < Duration::zero() { "-" } else { "" };
    let days = Duration::days(diff.num_days());
    let hours = Duration::hours((diff - days).num_hours());
    let minutes = Duration::minutes((diff - days - hours).num_minutes());
    let seconds = (diff - days - hours - minutes).num_seconds();

    println!(
        "{}{} days, {:02}:{:02}:{:02}",
        neg,
        days.num_days().abs(),
        hours.num_hours().abs(),
        minutes.num_minutes().abs(),
        seconds.abs(),
    );
}
