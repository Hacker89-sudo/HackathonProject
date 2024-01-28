use std::collections::HashMap;
use std::mem::transmute;

use super::value::Value;
use super::ast::Ast;

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

    pub fn eval_function(&mut self, name : &String, args : Vec<Value>) -> Value {
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
            _ => {},
        };

        
        if self.functions.len() + 128 >= self.functions.capacity() {
            // such a hack
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