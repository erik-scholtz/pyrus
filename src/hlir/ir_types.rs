use std::collections::HashMap;

use crate::ast::FuncAttributes;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    Color,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Color(String),
}

// IDs

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FuncId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GlobalId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValueId(pub u32);

use std::fmt;

impl fmt::Display for ValueId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Id {
    Func(FuncId),
    Global(GlobalId),
    Value(ValueId),
}

// default variables like the authur or default font size

pub enum Value {
    Literal(Literal),
    Ref(ValueId),
}

// rest of values expressed as expressions, I think this makes sense for now

#[derive(Debug, Clone)]
pub enum Op {
    Const {
        // TODO will probably need a table for internal reference where vars are used/called on instead of a name field
        result: ValueId,
        literal: Literal,
        ty: Type,
    },

    Var {
        result: ValueId,
        name: String,
        ty: Type,
    },

    Binary {
        result: ValueId,
        op: BinOp,
        lhs: ValueId,
        rhs: ValueId,
    },

    Call {
        result: Option<ValueId>,
        func: Id,
        args: Vec<ValueId>, // see if this needs to be a CallArg struct
        attributes: FuncAttributes,
    },

    Return {
        value: Option<ValueId>,
    },

    TextRef {
        index: usize,
    },
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
}

// how globals + Variables are handled

#[derive(Debug, Clone)]
pub struct Global {
    pub id: GlobalId,
    pub name: String,
    pub ty: Type,
    pub init: Literal,
}

pub struct Local {
    pub id: ValueId,
    pub ty: Type,
}

// how functions are handled

#[derive(Debug, Clone)]
pub struct Func {
    pub id: FuncId,
    pub name: String,
    pub args: Vec<Type>,
    pub attributes: FuncAttributes,
    pub return_type: Option<Type>,
    pub body: Block,
}

// how template, document and style sections are handled

#[derive(Debug, Clone)]
pub struct Block {
    pub ops: Vec<Op>,
    pub text: Vec<TextElement>,
}

#[derive(Debug, Clone)]
pub enum TextElement {
    Paragraph(String),
    Heading(String),
    List(Vec<TextElement>),
    Table(Vec<Vec<TextElement>>),
    Code(String, String), // Language, actual code
}

#[derive(Debug, Clone)]
pub struct HLIRModule {
    pub globals: HashMap<GlobalId, Global>, // TODO eventually remove IDs from actual struct and just refer to them (I think)
    pub functions: HashMap<FuncId, Func>,
}
