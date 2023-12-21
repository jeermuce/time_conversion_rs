use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'timeConversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */
enum Meridiem {
    AM,
    PM,
}

struct ParsedAmPmTime {
    hours: u8,
    minutes: u8,
    seconds: u8,
    meridiem: Meridiem,
}

fn parse_time(s: &str) -> ParsedAmPmTime {
    let parts: Vec<&str> = s.split(":").collect();
    let error_hours = format!("Error parsing hours {}", parts[0]);
    let hours: u8 = parts[0].parse().expect(error_hours.as_str());
    let error_minutes = format!("Error parsing minutes {}", parts[1]);
    let minutes: u8 = parts[1].parse().expect(error_minutes.as_str());
    let error_seconds = format!("Error parsing seconds {}", parts[2]);
    let seconds: u8 = parts[2][..2].parse().expect(error_seconds.as_str());
    match hours {
        1..=12 => {}
        _ => panic!("Invalid hours: {}", hours),
    }
    match minutes {
        0..=59 => {}
        _ => panic!("Invalid minutes: {}", minutes),
    }
    match seconds {
        0..=59 => {}
        _ => panic!("Invalid seconds: {}", seconds),
    }
    let meridiem = match parts[2].chars().nth(2).unwrap() {
        'A' => Meridiem::AM,
        'P' => Meridiem::PM,
        _ => panic!("Invalid meridiem: {}", parts[2]),
    };

    return ParsedAmPmTime {
        hours,
        minutes,
        seconds,
        meridiem,
    };
}

fn timeConversion(s: &str) -> String {
    let parsed_time = parse_time(s);
    let mut hours = parsed_time.hours;
    if let Meridiem::PM = parsed_time.meridiem {
        if hours != 12 {
            hours += 12;
        }
    } else {
        if hours == 12 {
            hours = 0;
        }
    }
    let hours_str = format!("{:02}", hours);
    let minutes_str = format!("{:02}", parsed_time.minutes);
    let seconds_str = format!("{:02}", parsed_time.seconds);
    return format!("{}:{}:{}", hours_str, minutes_str, seconds_str);
}

fn main() {
    let samples = vec![
        "07:05:45PM",
        "12:00:00AM",
        "12:00:00PM",
        "01:00:00AM",
        "01:00:00PM",
        "11:59:59PM",
    ];

    for sample in samples {
        println!("{} -> {}", sample, timeConversion(sample));
    }
}
