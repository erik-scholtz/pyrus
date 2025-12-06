use std::env;
use std::ffi::OsString;
use std::fs;

use pyrus::parser;
use pyrus::lexer;
use pyrus::ast;

fn main() {
    let args: Vec<OsString> = env::args_os().collect();

    println!("All args: {:?}", args);

    // Example: access the first argument (after program name)
    if args.len() > 1 {
        let first_arg = &args[1];
        println!("First argument: {:?}", first_arg);
    } else {
        println!("No arguments provided!");
    }

    let data = fs::read_to_string("temp.ink").expect("Should be able to read hosts file");

    // let tokens = lexer::lex(&data);
    // println!("{:?}", &tokens);

    let ast = parser::parse(&data);
    println!("{:#?}", ast);

}
