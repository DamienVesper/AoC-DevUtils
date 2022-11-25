extern crate dotenv;

use dotenv::dotenv;
use std::env;

use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, CONTENT_TYPE};

#[tokio::main]
async fn main() {
    // Load environment variables and arguments.
    dotenv().ok();
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("Correct execution format is: aoc-devutils.exe <year=int> <day=int> <mode=GET,SOLVE>");
        return;
    }

    let ref year = &args[1];
    let ref day = &args[2];
    let ref mode = &args[3];

    let possible_modes = ["GET", "SOLVE"];
    let ref mode1 = &possible_modes[0];
    let ref mode2 = &possible_modes[1];


    if mode != mode1 && mode != mode2 {
        println!("Correct execution format is: aoc-devutils.exe <day=int> <mode=GET,SOLVE>");
        return;
    }

    println!("{}", args.join(" "));

    // Get session secret from .env file.
    let mut session_secret = "".to_string();
    for (key, value) in env::vars() {
        if key == "SESSION_SECRET" {
            session_secret = value;
        }
    }

    if mode == mode1 {
        let res = reqwest::blocking::get("https://adventofcode.com/2020/day/1/input");
    } else {

    }
}
