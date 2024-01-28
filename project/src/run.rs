#![allow(unused)]

use super::parser::{grammar, PR};
use super::lexer::rules;
use super::environment::Environment;

use santiago::grammar::Grammar;
use santiago::lexer::LexerRules;

pub struct Runtime {
    parser : Grammar<PR>,
    lexer : LexerRules,
    environment : Environment,
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            parser: grammar(),
            lexer: rules(),
            environment: Environment::new(),
        }
    }

    pub fn run(&mut self, text : &str) -> () {

        let lexemes = santiago::lexer::lex(&self.lexer, &text).unwrap();
        let trees = santiago::parser::parse(&self.parser, &lexemes).unwrap();
        let mut tree = trees[0].as_abstract_syntax_tree();

        for i in tree.force_vast() {
            i.eval(&mut self.environment);
        }
    }
}

#[test]
fn test() -> () {
    let mut r = Runtime::new();

    r.run(
r###"fn fib(x) {
    x <= 2 : x,
    fib(x-1) + fib(x-2)
};

x = fib(8);
displn(x);

a = 1;
b = 1;

c = 1;

while c < 100 : [
    a = a + b;
    b = a + b;
    displn(a);
    displn(b);
    c = c + 1;
];

a = $[1, 2, 3];

displn(a);

a = a + 4 + 5 + 6;

displn(a);

b = $[1] + $[2];
displn(b);

a = "Hello,";
b = " World";
c = a + b;

displn("" + 17);

displn(c);

displn(1/0.0000001);

"###
    )
}