use std::env;
use std::ffi::OsString;
use std::fs;
use std::time::Instant;

use pyrus::ast;
use pyrus::hlir;
use pyrus::lexer;
use pyrus::parser;

fn main() {
    let last = Instant::now();
    let args: Vec<OsString> = env::args_os().collect();

    println!("All args: {:?}", args);

    if args.len() > 1 {
        let first_arg = &args[1];
        println!("First argument: {:?}", first_arg);
    } else {
        println!("No arguments provided!");
    }

    let data = fs::read_to_string("temp.ink").expect("Should be able to read hosts file");

    let tokens = lexer::lex(&data);
    // println!("{:?}", &tokens);

    let ast = parser::parse(tokens);
    // println!("{:#?}", ast);

    let hlir_module = hlir::lower(&ast);
    println!("{:#?}", hlir_module);

    let now = Instant::now();
    let time = now - last;
    println!("Time taken: {:?}", time);
}
