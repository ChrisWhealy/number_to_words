mod utils;

use std::{env, process::ExitCode};
use utils::number_to_text;

const LOWER_LIMIT: i32 = 0;
const UPPER_LIMIT: i32 = 999999999;

fn exit_with_msg_and_code(exit_code: ExitCode) -> impl Fn(&str) -> ExitCode {
    move |msg| {
        println!("{}", msg);
        exit_code
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn main() -> ExitCode {
    let exit_with_success = exit_with_msg_and_code(ExitCode::from(0));
    let exit_with_error = exit_with_msg_and_code(ExitCode::from(1));
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        exit_with_error(&format!(
            "Usage: number-to-words <a number between {} and {}>",
            LOWER_LIMIT, UPPER_LIMIT
        ))
    } else {
        if let Ok(this_num) = args[1].parse() {
            if this_num < LOWER_LIMIT || this_num > UPPER_LIMIT {
                exit_with_error(&format!(
                    "Error: Number must be between {} and {}",
                    LOWER_LIMIT, UPPER_LIMIT
                ))
            } else {
                exit_with_success(&format!("{}", number_to_text(this_num)))
            }
        } else {
            exit_with_error(&format!(
                "Error: {} cannot be parsed as an integer value",
                args[1]
            ))
        }
    }
}
