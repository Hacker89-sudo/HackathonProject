use super::value::Value;
use super::parser::PR;

pub type Name = String;
pub type Bast = Box<Ast>;
pub type Vast = Vec<Ast>;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash)]
pub enum Forkop {
    Gt,
    Lt,
    Lte,
    Gte,
    Eql,
    Neq,
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
}

#[derive(Debug, Clone)]
pub enum Ast {
    Literal         (Value),
    Get             (Name),
    VecLiteral      (Vast),
    FunDef          {name : Name, args : Vec<Name>, body : Bast},
    Set             {name : Name, value : Bast},
    While           {cond : Bast, body : Bast},
    ExpressionList  (Vast),
    Call            {name : Name, with : Vast},
    If              {if_ : Bast, then : Bast, else_ : Option<Bast>},
    Fork            {left : Bast, right : Bast, op : Forkop},
}

impl Ast {
    pub const NULL : Ast = Ast::Literal(Value::Nil);

    pub fn boxed(self) -> Box<Ast> {
        Box::new(self)
    }

    pub fn pr(self) -> PR {
        PR::Ast(self)
    }
}