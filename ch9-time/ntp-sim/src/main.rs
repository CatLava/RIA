use chrono::Local;
use chrono::DateTime;
extern crate clap;
use clap::{App, Arg};

fn simple_time_now() {
    let now = Local::now();
    println!("Hello, world!: {}", now);
}

struct Clock;

impl Clock {
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
        .about(" Gets and sets time")
        .arg(
            Arg::new("action")
                .get_possible_values(["get", "set"])
                .default_value("get"),
        )
        .arg(
            Arg::new("std")
                .short("s")
                .long("use-standard")
                .get_possible_values([
                    "rfc2822",
                    "rfc3339",
                    "timestamp",
                ])
                .default_value("rfc3339"),
        )
        .arg(Arg::new("datetime").help(
            "when action is set, apply date time"
        ));
    
    let args = app.get_matches();

    let action = args.value_of("action").unwrap();
    let std = args.value_of("std").unwrap();

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
