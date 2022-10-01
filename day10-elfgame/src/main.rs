use std::env;

fn main() {
    let mut arguments = env::args();

    match arguments.len() {
        0 | 1 => {
            println!("Missing argument: puzzle input.")
            return
        }
        _ => {
            let input = arguments.nth(1);
        }
    }
}
