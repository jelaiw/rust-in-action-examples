use chrono::{DateTime, Local, TimeZone};
use clap::{App, Arg};

use std::mem::zeroed;

// A struct with no fields is known as a zero-sized type or ZST.
// It does not occupy any memory in the resulting application and is purely a compile-time construct. 
struct Clock;

impl Clock {
    // DateTime<Local> is a DateTime with the Local time zone information.
    fn get() -> DateTime<Local> {
        Local::now()
    }

    fn set<Tz: TimeZone>(t: DateTime<Tz>) -> () {
        use libc::{timeval, time_t, suseconds_t};
        use libc::{settimeofday, timezone};

        let t = t.with_timezone(&Local);
        let mut u: timeval = unsafe { zeroed() };

        u.tv_sec = t.timestamp() as time_t;
        u.tv_usec = t.timestamp_subsec_micros() as suseconds_t;

        unsafe {
            // The timezone parameter of settimeofday() appears to be some sort of historic accident.
            // Non-null values generate an error.
            let mock_tz: *const timezone = std::ptr::null();
            settimeofday(&u as *const timeval, mock_tz);
        }
    }
}

fn main() {
    let app = App::new("clock")
        .version("0.1")
        .about("Gets and (aspirationally) sets the time.")
        .arg(
            Arg::with_name("action")
                .takes_value(true)
                .possible_values(&["get", "set"])
                .default_value("get")
        )
        .arg(
            Arg::with_name("std")
                .short("s")
                .long("use-standard")
                .takes_value(true)
                .possible_values(&[
                    "rfc2822",
                    "rfc3339",
                    "timestamp",
                ])
                .default_value("rfc3339")
        )
        .arg(
            Arg::with_name("datetime")
                .help("When <action> is 'set', apply <datetime>. \
                    Otherwise, ignore.")
        );

        let args = app.get_matches();

        // Supplies a default value to each argument via default_value("get") and default_value("rfc3339").
        // It’s safe to call unwrap() on these two lines.
        let action = args.value_of("action").unwrap();
        let std = args.value_of("std").unwrap();

        // Aborts early as we’re not ready to set the time yet.
        if action == "set" {
            unimplemented!()
        }

        let now = Clock::get();
        match std {
            "timestamp" => println!("{}", now.timestamp()),
            "rfc2822" => println!("{}", now.to_rfc2822()),
            "rfc3339" => println!("{}", now.to_rfc3339()),
            _ => unreachable!(),
        }
}
