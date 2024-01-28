#![allow(unused)]

use super::ast::{Ast, Forkop};
use super::value::Value;
use super::environment::Environment;
use super::error::error;

impl Ast {
    pub fn eval(&self, env : &mut Environment) -> Value {
        match self {
            Ast::Literal(l) => l.clone(),
            Ast::Get(g) => match env.fetch(g) {
                Some(x) => x,
                None => error(format!("Variable: {} Not Found", g)),
            },
            Ast::VecLiteral(v) => {
                let mut ret = Vec::with_capacity(v.len());
                for i in v {
                    ret.push(i.eval(env));
                }
                Value::List(ret)
            },
            Ast::FunDef { name, args, body } => {
                env.push_function(name, args, body);
                Value::Nil
            },
            Ast::Set { name, value } => {
                let e = value.eval(env);
                env.push_val(name, e);
                Value::Nil
            },
            Ast::While { cond, body } => {
                let mut v = Value::Nil;

                while cond.eval(env).force_bool() {
                    v = body.eval(env);
                }

                v
            },
            Ast::ExpressionList(e) => {
                let mut v = Value::Nil;

                for i in e {
                    v = i.eval(env);
                }

                v
            },
            Ast::Call { name, with } => {
                let mut evals = Vec::with_capacity(with.len());

                for i in with {
                    evals.push(i.eval(env));
                }

                env.eval_function(name, evals)
            },
            Ast::If { if_, then, else_ } => {
                if if_.eval(env).force_bool() {
                    return then.eval(env);
                }

                match else_ {
                    Some(x) => x.eval(env),
                    None => Value::Nil,
                }
            },
            Ast::Fork { left, right, op } => {
                let left = left.eval(env);
                let right = right.eval(env);

                match op {
                    Forkop::Gt  => {match left.comp(right) { Some (x) => Value::Bool(x > 0), None => error(format!("Invalid Comparison!"))}},
                    Forkop::Lt  => {match left.comp(right) { Some (x) => Value::Bool(x < 0), None => error(format!("Invalid Comparison!"))}},
                    Forkop::Lte => {match left.comp(right) { Some (x) => Value::Bool(x <= 0), None => error(format!("Invalid Comparison!"))}},
                    Forkop::Gte => {match left.comp(right) { Some (x) => Value::Bool(x >= 0), None => error(format!("Invalid Comparison!"))}},
                    Forkop::Eql => {match left.comp(right) { Some (x) => Value::Bool(x == 0), None => error(format!("Invalid Comparison!"))}},
                    Forkop::Neq => {match left.comp(right) { Some (x) => Value::Bool(x != 0), None => error(format!("Invalid Comparison!"))}},
                    
                    Forkop::Add => {left.add(right) },
                    Forkop::Sub => {left.sub(right) },
                    Forkop::Mul => {left.mul(right) },
                    Forkop::Div => {left.div(right) },
                    Forkop::Pow => {left.pow(right) },
                    Forkop::Mod => {left.mod_(right)},
                }
            },
        }
    }
}