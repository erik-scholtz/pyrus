use std::collections::HashMap;
use std::str::FromStr;

use crate::ast::{DocElement, Expression};

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
pub struct FuncId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GlobalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValueId(pub usize);

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

pub enum Value {
    Literal(Literal),
    Ref(ValueId),
}

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
    },
    Return {
        doc_element_ref: usize,
    },
    DocElementEmit {
        index: usize,
        attributes_ref: usize,
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
    pub return_type: Option<Type>,
    pub body: FuncBlock,
}

#[derive(Debug, Clone)]
pub struct FuncBlock {
    pub ops: Vec<Op>,
    pub returned_element_ref: usize,
}

// how template, document and style sections are handled

#[derive(Debug, Clone)]
pub struct Block {
    pub ops: Vec<Op>,
    pub element_refs: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct HLIRModule {
    pub globals: HashMap<GlobalId, Global>, // TODO eventually remove IDs from actual struct and just refer to them (I think)
    pub functions: HashMap<FuncId, Func>,
    pub attributes: AttributeTree,
    pub elements: Vec<DocElement>,
}

#[derive(Debug, Clone)]
pub struct AttributeTree {
    pub root: AttributeNode,
    pub size: usize,
}

impl AttributeTree {
    // TODO, will need to rething this ID stuff at some point but working for now
    pub fn new() -> Self {
        Self {
            root: AttributeNode::new(),
            size: 1,
        }
    }

    pub fn add_attribute(&mut self, attributes: AttributeNode) -> usize {
        let id = self.size;
        self.size += 1;
        self.root.add_child(attributes, id)
    }
}

#[derive(Debug, Clone)]
pub struct AttributeNode {
    pub parent: Option<usize>, // is this a pointer to another AttributeNode?
    pub id: usize,
    pub value: StyleAttributes,
    pub children: HashMap<usize, AttributeNode>,
}

impl AttributeNode {
    pub fn new() -> Self {
        Self {
            parent: None,
            id: 1,
            value: StyleAttributes::default(),
            children: HashMap::new(),
        }
    }

    pub fn new_with_attributes(attributes: &HashMap<String, Expression>, parent_id: usize) -> Self {
        Self {
            parent: Some(parent_id),
            id: parent_id + 1,
            value: StyleAttributes::new_with_attributes(attributes),
            children: HashMap::new(),
        }
    }

    pub fn add_child(&mut self, child: AttributeNode, parent_id: usize) -> usize {
        let id = parent_id + 1;
        self.children.insert(id, child);
        id
    }
}

// ----------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum Align {
    Left,
    Center,
    Right,
}

impl FromStr for Align {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "left" => Ok(Align::Left),
            "center" => Ok(Align::Center),
            "right" => Ok(Align::Right),
            _ => Err(format!("Invalid alignment value: {}", s)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PageBreak {
    Before,
    After,
    None,
}

impl FromStr for PageBreak {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "before" => Ok(PageBreak::Before),
            "after" => Ok(PageBreak::After),
            "none" => Ok(PageBreak::None),
            _ => Err(format!("Invalid page break value: {}", s)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StyleAttributes {
    // JS-like identity & styling
    pub id: Option<String>,
    pub class: Vec<String>,
    pub style: HashMap<String, String>,

    // Layout
    pub margin: Option<f32>,
    pub padding: Option<f32>,
    pub align: Option<Align>,

    // Rendering control
    pub hidden: bool,
    pub condition: Option<bool>, // corresponds to `if=...`

    // Pagination (PDF-specific)
    pub page_break: PageBreak,

    // Semantics
    pub role: Option<String>,
}

impl Default for StyleAttributes {
    fn default() -> Self {
        Self {
            id: None,
            class: Vec::new(),
            style: HashMap::new(),

            margin: None,
            padding: None,
            align: None,

            hidden: false,
            condition: None,

            page_break: PageBreak::None,

            role: None,
        }
    }
}
impl StyleAttributes {
    pub fn new_with_attributes(attributes: &HashMap<String, Expression>) -> Self {
        let mut result = Self::default();

        if let Some(expr) = attributes.get("id") {
            result.id = Some(expr.to_string());
        }

        if let Some(expr) = attributes.get("class") {
            result.class = expr
                .to_string()
                .split_whitespace()
                .map(String::from)
                .collect();
        }

        if let Some(expr) = attributes.get("style") {
            result.style = Self::parse_style(&expr.to_string());
        }

        if let Some(expr) = attributes.get("margin") {
            result.margin = expr.to_string().parse().ok();
        }

        if let Some(expr) = attributes.get("padding") {
            result.padding = expr.to_string().parse().ok();
        }

        if let Some(expr) = attributes.get("align") {
            result.align = expr.to_string().parse().ok();
        }

        if let Some(expr) = attributes.get("hidden") {
            result.hidden = expr.to_string().parse().unwrap_or(false);
        }

        if let Some(expr) = attributes.get("condition") {
            result.condition = expr.to_string().parse().ok();
        }

        if let Some(expr) = attributes.get("page_break") {
            result.page_break = expr.to_string().parse().unwrap_or(PageBreak::None);
        }

        if let Some(expr) = attributes.get("role") {
            result.role = Some(expr.to_string());
        }

        result
    }

    fn parse_style(input: &str) -> HashMap<String, String> {
        input
            .split(';')
            .filter_map(|decl| {
                let (key, value) = decl.split_once(':')?;
                Some((key.trim().to_string(), value.trim().to_string()))
            })
            .collect()
    }
}

// ----------------------------------------------------------------------------------
