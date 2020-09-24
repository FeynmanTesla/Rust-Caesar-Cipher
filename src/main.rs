use std::env;

mod front_end;
mod cli;

// TODO: make unit tests

/// Main method and entry point into the program.
/// Examine the first argument, corresponding to whether to use the CLI or GUI.
/// Depending on which interface is chosen, run the frontend or perform the described CLI operation from arguments from the user.
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