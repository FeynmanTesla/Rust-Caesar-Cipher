use std::env;

mod front_end;
mod cli;

// TODO: doc comments
// TODO: make unit tests

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        cli::print_usage();
        return;
    }

    let gui: bool = &args[1] == "gui";
    if gui {
        front_end::run_front_end().expect("Frontend failed to run");
    } else {
        cli::run_cli();
    }
}