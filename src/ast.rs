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
    Literal(String), // for simplicity, all literals are strings
    Identifier(String), // variable names
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
    Assignment {
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
    Function {
        name: String,
        params: Vec<FunctionParam>, // probably empty for now
        body: Vec<Statement>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expression>,
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

