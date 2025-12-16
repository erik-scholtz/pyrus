#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equals,
    Mod,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Negate,
    Not,
}

#[derive(Debug, Clone)]
pub enum Expression {
    StringLiteral(String),
    InterpolatedString(Vec<InterpPart>),
    NumberLiteral(i64),
    Identifier(String),
    Binary {
        left: Box<Expression>,
        operator: BinaryOp,
        right: Box<Expression>,
    },
    Unary {
        operator: UnaryOp,
        expression: Box<Expression>,
    },
}

impl Expression {
    pub fn as_number(&self) -> Option<i64> {
        match self {
            Expression::NumberLiteral(n) => Some(*n),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
enum InterpPart {
    Text(String),
    Expression(Expression), // interpolated portion
}

#[derive(Debug, Clone)]
pub struct FunctionParam {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct TemplateBlock {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct KeyValue {
    pub key: String,
    pub value: Expression,
}

#[derive(Debug, Clone)]
pub enum Statement {
    /// everything between `{` and `}` that isn't a function definition or a return
    DefaultSet {
        key: String,
        value: Expression,
    },
    VarAssign {
        // value should never be an expression, should always be explicit
        name: String,
        value: Expression,
    },
    ConstAssign {
        name: String,
        value: Expression,
    },
    If {
        condition: Expression,
        body: Vec<Statement>,
        else_body: Option<Vec<Statement>>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    For {
        iterator: String,
        iterable: Expression,
        body: Vec<Statement>,
    },
    Return(Expression),
    /// name(args) { body... }
    FunctionDecl {
        name: String,
        params: Vec<FunctionParam>, // probably empty for now
        body: Vec<Statement>,
    },
    FunctionCall {
        name: String,
        args: Vec<KeyValue>,
        attributes: Vec<KeyValue>,
    },
    Attributes {
        attributes: Vec<Statement>,
    },
    Paragraph {
        // literally just a block of text
        value: Expression,
    },
}

// Document Block

#[derive(Debug, Clone)]
pub struct DocumentBlock {
    pub statements: Vec<Statement>, // TODO document statements
}

#[derive(Debug, Clone)]
pub struct StyleBlock {
    pub statements: Vec<Statement>, // TODO style statements
}

#[derive(Debug, Clone)]
pub struct Ast {
    pub template: Option<TemplateBlock>,
    pub document: Option<DocumentBlock>,
    pub style: Option<StyleBlock>,
}
