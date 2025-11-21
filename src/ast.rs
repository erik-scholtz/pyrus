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
    pub statements: Vec<TemplateStatement>,
}

#[derive(Debug)]
pub enum TemplateStatement {
    /// everything between `{` and `}` that isn't a function definition or a return
    Assignment {
        name: String,
        value: Expression,
    },
    If {
        condition: Expression,
        body: Vec<TemplateStatement>,
        else_body: Option<Vec<TemplateStatement>>,
    },
    While {
        condition: Expression,
        body: Vec<TemplateStatement>,
    },
    For {
        iterator: String,
        iterable: Expression,
        body: Vec<TemplateStatement>,
    },
    Return(Expression),
    /// name(args) { body... }
    Function {
        name: String,
        params: Vec<FunctionParam>, // probably empty for now
        body: Vec<TemplateStatement>,
    },
}


// Document Block


#[derive(Debug)] 
pub struct DocumentBlock {
    pub statements: Vec<TemplateStatement>, // TODO document statements
}



#[derive(Debug)] 
pub struct StyleBlock {
    pub statements: Vec<TemplateStatement>, // TODO style statements
}

#[derive(Debug)]
pub struct Ast {
    pub template: Option<TemplateBlock>,
    pub document: Option<DocumentBlock>,
    pub style: Option<StyleBlock>,
}

