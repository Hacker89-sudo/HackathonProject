use std::collections::HashMap;
use std::mem::transmute;

use super::value::Value;
use super::ast::Ast;
use super::error::error;

use rug::Integer;

pub struct Environment {
    pub envs : Vec<HashMap<String, Value>>,
    pub functions : HashMap<String, (Vec<String>, Ast)>,
}

impl Environment {
    pub fn new() -> Environment {
        let mut ret = Environment { envs : Vec::new(), functions : HashMap::new() };
        ret.push_env();
        return ret;
    }

    pub fn wenv(&self) -> usize {
        self.envs.len() - 1
    }

    pub fn push_env(&mut self) -> () {
        self.envs.push(HashMap::new())
    }

    pub fn push_val(&mut self, name : &String, val : Value) -> () {
        if self.envs[0].contains_key(name) {
            self.envs[0].insert(name.clone(), val);
        }
        else {
            let s = self.wenv();
            self.envs[s].insert(name.clone(), val);
        }
    }

    pub fn force_push(&mut self, name : &String, val : Value) -> () {
        let c = self.wenv();
        self.envs[c].insert(name.clone(), val);
    }

    pub fn push_function(&mut self, name : &String, args : &Vec<String>, val : &Ast) -> () {
        self.functions.insert(name.clone(), (args.clone(), val.clone()));
    }

    pub fn eval_function(&mut self, name : &String, mut args : Vec<Value>) -> Value {
        args.reverse(); // there was a mistake in the grammar and it's too late now!

        match name.as_str() {
            "disp" => {
                for i in args {
                    print!("{}", i);
                }
                return Value::Nil;
            },
            "displn" => {
                for i in args {
                    println!("{}", i);
                }
                return Value::Nil;
            },
            "pop" => {
                if args.len() != 2 {
                    return error(format!("Calls to pop should have 2 arguments : a vector followed by a number"));
                }

                let t = args[1].take().as_integer();

                if !args[0].is_vec() || !t.is_integer() {
                    println!("{}", args[0]);
                    return error(format!("Calls to pop should have 2 arguments : a vector followed by a number"));
                }

                let Value::List(mut l) = args[0].take() else {unreachable!()};

                l.remove(t.force_integer().unwrap().to_usize_wrapping());

                return Value::List(l);
            },
            "get" => {
                if args.len() != 2 {
                    return error(format!("Calls to get should have 2 arguments : a vector followed by a number"));
                }

                let t = args[1].take().as_integer();

                if !args[0].is_vec() || !t.is_integer() {
                    return error(format!("Calls to get should have 2 arguments : a vector followed by a number"));
                }

                let Value::List(l) = args[0].take() else {unreachable!()};

                return match l.get(t.force_integer().unwrap().to_usize_wrapping()) {
                    Some(x) => x.clone(),
                    None => error(format!("get called with out of bounds index")),
                };
            },
            "dim" => {
                if args.len() != 1 {
                    return error(format!("Calls to dim should have ONE arguments"));
                }

                return match args[0].take() {
                    Value::Float(_) => Value::Integer(Integer::new()),
                    Value::Integer(_) => Value::Integer(Integer::new()),
                    Value::Bool(_) => Value::Integer(Integer::new()),
                    Value::String(s) => Value::Integer(Integer::from(s.len())),
                    Value::List(l) => Value::Integer(Integer::from(l.len())),
                    Value::Nil => Value::Integer(Integer::new()),
                };
            },
            "vec" => {
                if args.len() != 1 {
                    return error(format!("Calls to vec should have ONE arguments"));
                }

                return args[0].take().as_lst();
            },
            "str" => {
                if args.len() != 1 {
                    return error(format!("Calls to str should have ONE arguments"));
                }

                return args[0].take().as_str();
            },
            "int" => {
                if args.len() != 1 {
                    return error(format!("Calls to int should have ONE arguments"));
                }

                return args[0].take().as_integer();
            },
            "nil" => {
                if args.len() != 1 {
                    return error(format!("Calls to nil should have ONE arguments"));
                }

                return match args[0].take() {
                    Value::Nil => Value::Bool(false),
                    _ => Value::Bool(true),
                };
            },
            _ => {},
        };

        
        if self.functions.len() + 128 >= self.functions.capacity() {
            // such a hack :/
            self.functions.reserve(256);
        }
        
        self.push_env();

        let Some((names, body)) = self.functions.get(name) else {return Value::Nil;};
        
        let names = names as *const Vec<String>;
        let body = body as *const Ast;
        
        // this is unsafe iff the source of functions moves  -- if a function is defined with the same name OR 128 to 256 functions are added
        // hopefully that doesn't happen
        let names = unsafe { transmute::<*const Vec<String>, &Vec<String>>(names) };
        let body = unsafe { transmute::<*const Ast, &Ast>(body) };
        

        for (s, v) in names.into_iter().zip(args) {
            self.force_push(s, v);
        }

        let ret = body.eval(self);

        self.pop_env();

        return ret;
    }

    pub fn fetch(&self, name : &String) -> Option<Value> {
        match self.envs[self.wenv()].get(name) {
            Some(x) => {return Some(x.clone());},
            None => {},
        };

        return match self.envs[0].get(name) {
            Some(x) => Some(x.clone()),
            None => None,
        };
    }

    pub fn pop_env(&mut self) -> () {
        self.envs.pop();
    }
}