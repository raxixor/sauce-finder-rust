#[macro_use]
extern crate smart_default;
use std::env;

mod iqdb;
mod util;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        main_input()
    } else {
        let res = util::build_match(args[1].as_ref());

        if let Ok(m) = res {
            println!("{}", m);

            if m.found.is_empty() {
                println!("Found zero results.");
            }
        } else if let Err(e) = res {
            println!("Error: {}", e)
        }
    }
}

fn main_input() {
    let args: String = dialoguer::Input::<String>::new()
        .allow_empty(false)
        .with_prompt("Link")
        .interact()
        .unwrap();

    let res = util::build_match(args.as_ref());

    if let Ok(m) = res {
        println!("{}", m);

        if m.found.is_empty() {
            println!("Found zero results.");
        }
    } else if let Err(e) = res {
        println!("Error: {}", e);
    }
}
