#![allow(unused)]

use super::ast::{Ast, Vast, Bast, Forkop};
use super::value::{Value, FLOATING_PRECISION};

use santiago::grammar::Associativity;
use santiago::grammar::Grammar;

use rug::Integer;
use rug::Float;

use std::str::FromStr;

#[derive(Debug)]
pub enum PR {
    NameList        (Vec<String>),
    Vast            (Vec<Ast>),
    Name            (String),

    Ast             (Ast),
    Literal         (Value),

    Marker,
    None,
}

impl PR {
    pub const EMPTY_VAST : PR = PR::Vast(vec![]);
    pub const EMPTY_NAMELIST : PR = PR::NameList(vec![]);

    pub fn take(&mut self) -> Self {
        let mut t = PR::None;
        std::mem::swap(&mut t, self);
        return t;
    }

    pub fn force_namelist(&mut self) -> Vec<String> {
        match self.take() {
            PR::NameList(x) => x,
            _ => panic!(),
        }
    }

    pub fn force_vast(&mut self) -> Vast {
        match self.take() {
            PR::Vast(x) => x,
            _ => panic!(),
        }
    }

    pub fn force_ast(&mut self) -> Ast {
        match self.take() {
            PR::Ast(x) => x,
            _ => panic!(),
        }
    }

    pub fn force_name(&mut self) -> String {
        match self.take() {
            PR::Name(x) => x,
            _ => panic!(),
        }
    }

    pub fn force_bast(&mut self) -> Box<Ast> {
        Box::new(self.force_ast())
    }
}

pub fn rev<T>(mut to_rev : Vec<T>) -> Vec<T> {
    to_rev.reverse();
    to_rev
}

pub fn grammar() -> Grammar<PR> {
    santiago::grammar!(

        // Structure
        "program"   => rules "expr_list"    => |mut t : Vec<PR>| {PR::Vast(rev(t[0].take().force_vast()))};

        "expr_list" => rules "expr" ";" "expr_list" =>|mut t : Vec<PR>| {let mut v = t[2].force_vast(); v.push(t[0].force_ast()); PR::Vast(v)};
        "expr_list" => empty                            => |_| {PR::Vast(Vec::new())};

        "expr"      => rules "(" "expr" ")" => |mut t| {t[1].take()};

        "expr"      => rules "name"         => |mut t| {Ast::Get(t[0].force_name()).pr()};
        "expr"      => rules "$" "[" "arg_list" "]" => |mut t| {Ast::VecLiteral(rev(t[2].force_vast())).pr()};
        "expr"      => rules "fn" "name" "(" "name_list" ")" "expr" => |mut t| {Ast::FunDef{args : rev(t[3].force_namelist()), body: t[5].force_bast(), name: t[1].force_name()}.pr()};
        "expr"      => rules "name" "=" "expr" => |mut t| {Ast::Set{name : t[0].force_name(), value : t[2].force_bast()}.pr()};
        
        "expr"      => rules "[" "expr_list" "]" => |mut t| {Ast::ExpressionList(rev(t[1].force_vast())).pr()};

        "name_list" => rules "name" "," "name_list" => |mut t| {let mut a = t[2].force_namelist(); a.push(t[0].force_name()); PR::NameList(a)};
        "name_list" => rules "name"                 => |mut t| {PR::NameList(vec![t[0].force_name()])};
        "name_list" => empty                        => |_|  {PR::EMPTY_NAMELIST};

        "expr"      => rules "name" "(" "arg_list" ")"  => |mut t| {Ast::Call{name : t[0].force_name(), with : t[2].force_vast()}.pr()};
        "arg_list"  => rules "expr" "," "arg_list"  => |mut t| {let mut v = t[2].force_vast(); v.push(t[0].force_ast()); PR::Vast(v)};
        "arg_list"  => rules "expr"                 => |mut t| {PR::Vast(vec![t[0].force_ast()])};
        "arg_list"  => empty                        => |_| {PR::EMPTY_VAST};

        "expr"       => rules "{" "conditional" "}"     => |mut t| {t[1].take()};
        "conditional"=> rules "expr" ":" "expr" "," "conditional" => |mut t| {Ast::If{else_ : Some(t[4].force_bast()), if_ : t[0].force_bast(), then : t[2].force_bast()}.pr()};
        "conditional"=> rules "expr" ":" "expr" => |mut t| {Ast::If{else_ : Some(Ast::NULL.boxed()), if_ : t[0].force_bast(), then : t[2].force_bast()}.pr()};
        "conditional"=> rules "expr" => |mut t| {t[0].take()};
        "conditional"=> empty       => |_| {Ast::NULL.pr()};

        "expr"      => rules "while" "expr" ":" "expr"  => |mut t| {Ast::While{body : t[3].force_bast(), cond : t[1].force_bast()}.pr()};

        "expr"      => rules "expr" "<" "expr" => |mut t| {Ast::Fork{left : t[0].force_bast(), right : t[2].force_bast(), op : Forkop::Lt}.pr()};
        "expr"      => rules "expr" ">" "expr" => |mut t| {Ast::Fork{left : t[0].force_bast(), right : t[2].force_bast(), op : Forkop::Gt}.pr()};
        "expr"      => rules "expr" "<=" "expr"=> |mut t| {Ast::Fork{left : t[0].force_bast(), right : t[2].force_bast(), op : Forkop::Lte}.pr()};
        "expr"      => rules "expr" ">=" "expr"=> |mut t| {Ast::Fork{left : t[0].force_bast(), right : t[2].force_bast(), op : Forkop::Gte}.pr()};
        "expr"      => rules "expr" "==" "expr"=> |mut t| {Ast::Fork{left : t[0].force_bast(), right : t[2].force_bast(), op : Forkop::Eql}.pr()};
        "expr"      => rules "expr" "!=" "expr"=> |mut t| {Ast::Fork{left : t[0].force_bast(), right : t[2].force_bast(), op : Forkop::Neq}.pr()};

        "expr"      => rules "expr" "+" "expr" => |mut t| {Ast::Fork{left : t[0].force_bast(), right : t[2].force_bast(), op : Forkop::Add}.pr()};
        "expr"      => rules "expr" "-" "expr" => |mut t| {Ast::Fork{left : t[0].force_bast(), right : t[2].force_bast(), op : Forkop::Sub}.pr()};
        "expr"      => rules "expr" "*" "expr" => |mut t| {Ast::Fork{left : t[0].force_bast(), right : t[2].force_bast(), op : Forkop::Mul}.pr()};
        "expr"      => rules "expr" "/" "expr" => |mut t| {Ast::Fork{left : t[0].force_bast(), right : t[2].force_bast(), op : Forkop::Div}.pr()};
        "expr"      => rules "expr" "^" "expr" => |mut t| {Ast::Fork{left : t[0].force_bast(), right : t[2].force_bast(), op : Forkop::Pow}.pr()};
        "expr"      => rules "expr" "%" "expr" => |mut t| {Ast::Fork{left : t[0].force_bast(), right : t[2].force_bast(), op : Forkop::Mod}.pr()};

        // Lexeme ports
        "name"      =>  lexemes "NAME"      => |t| { PR::Name(t[0].raw.clone()) };
        "expr"      =>  lexemes "STRING"    => |t| { PR::Ast({
            let x = t[0].raw.clone();
            let raw = x.as_str();
            let len = raw.len();
            let final_ = &raw[1..len-1];

            Ast::Literal(Value::String(String::from(final_)))
        }) };
        "expr"      =>  lexemes "INTEGER"   => |t| { PR::Ast(Ast::Literal(Value::Integer(rug::Integer::from_str(&t[0].raw.as_str()).unwrap()))) };
        "expr"      =>  lexemes "FLOAT"     => |t| { PR::Ast(Ast::Literal(Value::Float(
            {
                Float::with_val(FLOATING_PRECISION, Float::parse(&t[0].raw.as_str()).unwrap())
            }
        ))) };

        "="         =>  lexemes "="         => |_| { PR::Marker };
        "+"         =>  lexemes "+"         => |_| { PR::Marker };
        "-"         =>  lexemes "-"         => |_| { PR::Marker };
        "*"         =>  lexemes "*"         => |_| { PR::Marker };
        "/"         =>  lexemes "/"         => |_| { PR::Marker };
        "^"         =>  lexemes "^"         => |_| { PR::Marker };
        "%"         =>  lexemes "%"         => |_| { PR::Marker };
        "<"         =>  lexemes "<"         => |_| { PR::Marker };
        ">"         =>  lexemes ">"         => |_| { PR::Marker };
        "<="        =>  lexemes "<="        => |_| { PR::Marker };
        ">="        =>  lexemes ">="        => |_| { PR::Marker };
        "=="        =>  lexemes "=="        => |_| { PR::Marker };
        "!="        =>  lexemes "!="        => |_| { PR::Marker };

        ":"         =>  lexemes ":"         => |_| { PR::Marker };
        ","         =>  lexemes ","         => |_| { PR::Marker };
        "("         =>  lexemes "("         => |_| { PR::Marker };
        ")"         =>  lexemes ")"         => |_| { PR::Marker };
        "{"         =>  lexemes "{"         => |_| { PR::Marker };
        "}"         =>  lexemes "}"         => |_| { PR::Marker };
        "["         =>  lexemes "["         => |_| { PR::Marker };
        "]"         =>  lexemes "]"         => |_| { PR::Marker };
        "fn"        =>  lexemes "FN"        => |_| { PR::Marker };
        "$"         =>  lexemes "$"         => |_| { PR::Marker };
        ";"         =>  lexemes ";"         => |_| { PR::Marker };
        "while"     =>  lexemes "WHILE"     => |_| { PR::Marker };

        Associativity::Right => rules "=";
        Associativity::Left => rules "<=" ">=" "==" "!=" ">" "<";
        Associativity::Left => rules "+" "-";
        Associativity::Left => rules "*" "/";
        Associativity::Left => rules "%" "^";
    )
}

#[test]
fn test_me() -> () {
    let text = "

    fn fib(x) {
        x <= 2 : x,
        fib(x-1) + fib(x-2)
    };
    
    x = fib(8);
    
    a = 1;
    b = 1;
    
    while 1 : [
        a = a + b;
        b = a + b;
        disp(a);
        disp(b);
    ];
    
";

    use super::lexer::rules;

    let l = rules();
    let g = grammar();

    let lexemes = santiago::lexer::lex(&l, &text).unwrap();
    let trees = santiago::parser::parse(&g, &lexemes).unwrap();
    println!("LENGTH {}", trees.len());
    let tree = trees[0].as_abstract_syntax_tree();

    println!("{:#?}", tree);
}