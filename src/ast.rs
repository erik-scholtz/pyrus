#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equals,
}

#[derive(Debug)]
pub enum UnaryOp {
    Negate,
    Not,
}

#[derive(Debug)]
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

#[derive(Debug)]
enum InterpPart {
    Text(String),
    Expression(Expression),      // interpolated portion
}

#[derive(Debug)]
pub struct FunctionParam {
    pub name: String,
}

#[derive(Debug)]
pub struct TemplateBlock {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    /// everything between `{` and `}` that isn't a function definition or a return
    DefaultSet {
        name: String,
        value: Expression,
    },
    VarAssign {
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
        params: Vec<Statement>,
        attributes: Vec<Statement>,
    },
    KeyValue {
        key: String,
        value: Expression,
    },
    Attributes {
        attributes: Vec<Statement>,
    },
    Paragraph { // literally just a block of text
        value: Expression,
    },
}


// Document Block


#[derive(Debug)] 
pub struct DocumentBlock {
    pub statements: Vec<Statement>, // TODO document statements
}



#[derive(Debug)] 
pub struct StyleBlock {
    pub statements: Vec<Statement>, // TODO style statements
}

#[derive(Debug)]
pub struct Ast {
    pub template: Option<TemplateBlock>,
    pub document: Option<DocumentBlock>,
    pub style: Option<StyleBlock>,
}

