use chrono::{DateTime, Local};
use clap::{App, Arg};

// A struct with no fields is known as a zero-sized type or ZST.
// It does not occupy any memory in the resulting application and is purely a compile-time construct. 
struct Clock;

impl Clock {
    // DateTime<Local> is a DateTime with the Local time zone information.
    fn get() -> DateTime<Local> {
        Local::now()
    }

    fn set() -> ! {
        unimplemented!()
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
