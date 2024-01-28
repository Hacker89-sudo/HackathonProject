#![allow(unused)]

use std::cmp::min;
use rug::ops::Pow;
use rug::Float;
use rug::Integer;

use super::error::error;

pub const FLOATING_PRECISION : u32 = 363;

#[derive(Debug, Clone)]
pub enum Value {
    Float       (Float),
    Integer     (Integer),
    Bool        (bool),
    String      (String),
    List        (Vec<Value>),
    Nil,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Float(b) => write!(f, "{}", b),
            Value::Integer(b) => write!(f, "{}", b),
            Value::Bool(b) => {
                write!(f, "{}", b)
            },
            Value::String(s) => {
                write!(f, "{}", s)
            },
            Value::List(l) => {
                write!(f, "[")?;
                for e in l { write!(f, "{},", e)?; }
                write!(f, "]")
            },
            Value::Nil => write!(f, "nil"),
        }
        // write!(f, "({}, {})", self.longitude, self.latitude)
    }
}

impl Value {
    // zero is strongest
    fn strength(&self) -> u8 {
        match self {
            Value::Nil => 0,
            Value::List(_) => 1,
            Value::String(_) => 2,
            Value::Float(_) => 3,
            Value::Integer(_) => 4,
            Value::Bool(_) => 5,
        }
    }

    fn as_strength(self, s_tag : u8) -> Self {
        match s_tag {
            0 => Value::Nil,
            1 => self.as_lst(),
            2 => self.as_str(),
            3 => self.as_float(),
            4 => self.as_integer(),
            5 => self.as_bool(),
            _ => unreachable!(),
        }
    }

    /// convert both to the 'strongest type'\
    /// Strongest to Weakest (Nil is the Strongest): \
    ///     Nil \
    ///     List \
    ///     String\
    ///     Float  \
    ///     Integer \
    ///     Bool     
    pub fn strongest(self, other : Value) -> (Value, Value) {
        let strongest = min(self.strength(), other.strength());
        let (a,b) = (self.as_strength(strongest), other.as_strength(strongest));
        if a.is_nill() || b.is_nill() {
            return (Value::Nil, Value::Nil);
        }
        else {
            return (a,b);
        }
    }

    pub fn fzero() -> Float {
        Float::new(FLOATING_PRECISION)
    }

    /// guaranteed to return either a float type Value or a Nil type Value
    pub fn as_float(self) -> Value {
        match self {
            Value::Float(_) => self,
            Value::Integer(i) => {
                let mut f = Self::fzero();
                f += i;
                return Value::Float(f);
            },
            Value::Bool(b) => {
                if b {
                    let mut v = Self::fzero();
                    v += 1;
                    return Value::Float(v);
                }
                else {
                    Value::Float(Self::fzero())
                }
            },
            Value::String(..) => {
                return Value::Nil;
            },
            Value::List(..) => Value::Nil,
            Value::Nil => Value::Nil,
        }
    }

    /// guaranteed to return either a integer type Value or a Nil type Value
    pub fn as_integer(self) -> Value {
        match self {
            Value::Float(f) => Value::Integer(f.to_integer().unwrap_or_default()),
            Value::Integer(..) => self,
            Value::Bool(i) => Value::Integer(if i {Integer::from(1)} else {Integer::from(0)}),
            Value::String(_) => Value::Nil,
            Value::List(_) => Value::Nil,
            Value::Nil => Value::Nil,
        }
    }

    /// guaranteed to return either a bool type Value or a Nil type Value
    pub fn as_bool(self) -> Value {
        match self {
            Value::Float(f) => Value::Bool(f > 0),
            Value::Integer(i) => Value::Bool(i > 0),
            Value::Bool(..) => self,
            Value::String(s) => Value::Bool(s.len() > 0),
            Value::List(l) => Value::Bool(l.len() > 0),
            Value::Nil => Value::Nil,
        }
    }

    /// guaranteed to return either a str type Value or a Nil type Value
    pub fn as_str(self) -> Value {
        match self {
            Value::Float(f) => Value::String(format!("{}", f)),
            Value::Integer(f) => Value::String(format!("{}", f)),
            Value::Bool(f) => Value::String(format!("{}", f)),
            Value::String(_) => self,
            Value::List(f) => Value::String(format!("{:?}", f)),
            Value::Nil => Value::Nil,
        }
    }

    /// guaranteed to return either a lst type Value or a Nil type Value
    pub fn as_lst(self) -> Value {
        match self {
            Value::List(_) => self,
            Value::Nil => Value::Nil,
            _ => Value::List(vec![self]),
        }
    }

    pub fn force_bool(self) -> bool {
        match self.as_bool() {
            Value::Bool(x) => x,
            _ => {
                error(format!("Attempts to convert NILL to boolean"));
                false
            }
        }
    }

    pub fn is_nill(&self) -> bool {
        match self {
            _ => false,
            Value::Nil => true,
        }
    }

    pub fn is_integer(&self) -> bool {
        match self {
            Value::Integer(_) => true,
            _ => false,
        }
    }

    pub fn is_float(&self) -> bool {
        match self {
            Value::Float(_) => true,
            _ => false,
        }
    }

    pub fn force_float(self) -> Option<Float> {
        match self {
            Value::Float(f) => Some(f),
            _ => None,
        }
    }

    pub fn force_integer(self) -> Option<Integer> {
        match self {
            Value::Integer(i) => Some(i),
            _ => None,
        }
    }

    // self - other \
    // (rounded to -1, 0, or 1) \
    // returns None if conversion error
    pub fn comp(self, other : Value) -> Option<i8> {
        if !(self.is_comparable() && other.is_comparable()) {
            return None;
        }

        if self.is_float() || other.is_float() {
            let Some( one ) = self.as_float().force_float() else {return None;};
            let Some( two ) = other.as_float().force_float() else {return None;};
            
            if one < two {
                Some(-1)
            }
            else if one > two {
                Some(1)
            }
            else {
                Some(0)
            }
        }
        else {
            let Some( one ) = self.as_integer().force_integer() else {return None;};
            let Some( two ) = other.as_integer().force_integer() else {return None;};

            if one < two {
                Some(-1)
            }
            else if one > two {
                Some(1)
            }
            else {
                Some(0)
            }
        }
    }

    pub fn is_comparable(&self) -> bool {
        match self {
            &Value::Float(_) => true,
            &Value::Integer(_) => true,
            _ => false,
        }
    }

    pub fn add(self, other : Value) -> Value {
        let (l, r) = self.strongest(other);

        match (l, r) {
            (Value::Float(mut f1), Value::Float(f2)) => {f1 += f2; Value::Float(f1)},
            (Value::Integer(mut i1), Value::Integer(mut i2)) => {i1 += i2; Value::Integer(i1)},
            (Value::Bool(b1), Value::Bool(b2)) => {error(format!("Cannot add bool to bool"))},
            (Value::String(mut s1), Value::String(s2)) => {s1.extend(s2.chars()); Value::String(s1)},
            (Value::List(mut l1), Value::List(l2)) => {l1.extend(l2); Value::List(l1)},
            (Value::Nil, Value::Nil) => Value::Nil,
            (_, _)  => unreachable!(),
        }
    }

    pub fn sub(self, other : Value) -> Value {
        let (l, r) = self.strongest(other);

        match (l, r) {
            (Value::Float(mut f1), Value::Float(f2)) => {f1 -= f2; Value::Float(f1)},
            (Value::Integer(mut i1), Value::Integer(mut i2)) => {i1 -= i2; Value::Integer(i1)},
            (Value::Bool(b1), Value::Bool(b2)) => error(format!("Cannot sub bool from bool")),
            (Value::String(s1), Value::String(s2)) => error(format!("Cannot sub string")),
            (Value::List(l1), Value::List(l2)) => error(format!("Cannot sub list")),
            (Value::Nil, Value::Nil) => Value::Nil,
            (_, _)  => unreachable!(),
        }
    }

    pub fn mul(self, other : Value) -> Value {
        let (l, r) = self.strongest(other);

        match (l, r) {
            (Value::Float(mut f1), Value::Float(f2)) => {f1 *= f2; Value::Float(f1)},
            (Value::Integer(mut i1), Value::Integer(mut i2)) => {i1 *= i2; Value::Integer(i1)},
            (Value::Bool(b1), Value::Bool(b2)) => error(format!("Cannot mul bool and bool")),
            (Value::String(s1), Value::String(s2)) => error(format!("Cannot mul string")),
            (Value::List(l1), Value::List(l2)) => error(format!("Cannot mul list")),
            (Value::Nil, Value::Nil) => Value::Nil,
            (_, _)  => unreachable!(),
        }
    }

    pub fn div(self, other : Value) -> Value {
        let (l, r) = self.strongest(other);

        match (l, r) {
            (Value::Float(mut f1), Value::Float(f2)) => {if f2 == 0 {error(format!("division by zero!"))} else {f1 /= f2; Value::Float(f1)}},
            (Value::Integer(mut i1), Value::Integer(mut i2)) => {if i2 == 0 {return error(format!("division by zero!"));}; i1 /= i2; Value::Integer(i1)},
            (Value::Bool(b1), Value::Bool(b2)) => error(format!("Cannot div bool and bool")),
            (Value::String(s1), Value::String(s2)) => error(format!("Cannot div (by) string")),
            (Value::List(l1), Value::List(l2)) => error(format!("Cannot div (by) list")),
            (Value::Nil, Value::Nil) => Value::Nil,
            (_, _)  => unreachable!(),
        }
    }

    pub fn pow(self, other : Value) -> Value {
        let (l, r) = self.strongest(other);

        match (l, r) {
            (Value::Float(mut f1), Value::Float(f2)) => {f1 = f1.pow(f2); Value::Float(f1)},
            (Value::Integer(mut i1), Value::Integer(mut i2)) => {let t = i2.to_u32(); if t.is_none() {return error(format!("Exponent too big! (Or Small). To bypass this error first convert to float using float(arg)."));}; i1 = i1.pow(t.unwrap()); Value::Integer(i1)},
            (Value::Bool(b1), Value::Bool(b2)) => error(format!("Cannot raise bool to power")),
            (Value::String(s1), Value::String(s2)) => error(format!("Cannot raise String to Value or raise Value to String")),
            (Value::List(l1), Value::List(l2)) => error(format!("Cannot raise List to Value or raise Value to List")),
            (Value::Nil, Value::Nil) => Value::Nil,
            (_, _)  => unreachable!(),
        }
    }

    pub fn mod_(self, other : Value) -> Value {
        let (l, r) = self.strongest(other);

        match (l, r) {
            (Value::Float(mut f1), Value::Float(f2)) => {f1 %= f2; Value::Float(f1)},
            (Value::Integer(mut i1), Value::Integer(mut i2)) => {i1 %= i2; Value::Integer(i1)},
            (Value::Bool(b1), Value::Bool(b2)) => error(format!("Cannot mod bool and bool")),
            (Value::String(s1), Value::String(s2)) => error(format!("Cannot mod (by) string")),
            (Value::List(l1), Value::List(l2)) => error(format!("Cannot mod (by) list")),
            (Value::Nil, Value::Nil) => Value::Nil,
            (_, _)  => unreachable!(),
        }
    }
}