#![allow(unused)]

use super::parser::{grammar, PR};
use super::lexer::rules;
use super::environment::Environment;
use super::error::error;
use super::value::Value;

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

    pub fn run(&mut self, text : &str) -> Value {

        let Ok(lexemes) = santiago::lexer::lex(&self.lexer, &text) else {
            return error(format!("Invalid Syntax!"));
        };

        let Ok( trees ) = santiago::parser::parse(&self.parser, &lexemes) else {
            return error(format!("Invalid Structure of Program!"));
        };

        let mut tree = trees[0].as_abstract_syntax_tree();

        let mut v = Value::Nil;

        for i in tree.force_vast() {
            v = i.eval(&mut self.environment);
        }

        return v;
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

while c < 10 : [
    displn(a);
    displn(b);
    a = a + b;
    b = a + b;
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

displn(1/0.000000);

"###
    );
}