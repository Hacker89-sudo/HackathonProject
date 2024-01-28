mod lexer;
mod parser;
mod ast;
mod value;
mod eval;
mod environment;
mod error;
mod run;

use std::process::exit;
use run::Runtime;

fn main() -> () {
    let argv : Vec<String> = std::env::args().collect();

    if argv.len() > 2 {
        let c = argv[0].clone();
        println!("Incorrect usage of utility {c}.\nCorrect Usage:\n\t{c} # for command line util\n\t{c} FILENAME # to run file");
        exit(1);
    }

    let mut r = Runtime::new();

    if argv.len() == 1 {
        println!(r##"Shell Mode! Hello! 'Ctrl+C' to Exit"##);
        
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            println!("Result : {}", r.run(input.as_str()));
        }
    }
    else {
        let f = (String::from_utf8(std::fs::read(argv[1].as_str()).unwrap())).unwrap();
        r.run(f.as_str());
    }
}
