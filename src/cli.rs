use std::env;

use crate::front_end::state::logic::find_output;

pub(crate) fn run_cli() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        print_usage();
        return;
    }

    let encrypting: bool = &args[2] == "E";
    let shift: &String = &args[3];
    let shift_auto: bool = &args[3] == "A";
    let raw_text: String = String::from(&args[4..].join(" "));

    let shift_size: f64 = if shift_auto { -1.0 as f64 } else { (&shift).parse::<f64>().unwrap() as f64 };

    let res = find_output(encrypting, shift_auto, shift_size, raw_text);
    println!("{}", res);
}

pub(crate) fn print_usage() {
    println!("Usage: <\"gui\" or \"cli\"> <If using CLI: \"E\" for encrypt, \"D\" for decrypt> <If using CLI: shift size or \"A\" for a random number> <If using CLI: src text>.");
}