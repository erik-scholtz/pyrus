use std::collections::HashMap;

use crate::ast::Ast;

// default variables like the authur or default font size

#[derive(Debug, Clone)]
enum Type {
    Int,
    Float,
    String,
    Bool,
}

#[derive(Debug, Clone)]
enum ReturnType {
    Void,
    Type(Type),
}

#[derive(Debug, Clone)]
enum ConstValue {
    Int(i64),
    Float(f64),
    Color(String),
    String(String),
    Bool(bool),
}

#[derive(Debug, Clone)]
enum VarValue {
    Int(i64),
    Float(f64),
    Color(String),
    String(String),
    Bool(bool),
}

#[derive(Debug, Clone)]
enum Assign {
    Const(ConstValue),
    Var(VarValue),
}

// rest of values expressed as expressions, I think this makes sense for now

#[derive(Debug, Clone)]
enum Expr {
    VariableDecl(VariableDecl),
    BinaryOp(String, Box<Expr>, Box<Expr>),
    UnaryOp(String, Box<Expr>),
    Return(VarValue), // TODO this is most definitely not right
}

// how globals + Variables are handled

#[derive(Debug, Clone)]
struct GlobalDecl {
    name: String,
    ty: Type, // type but rust already uses that
    value: Assign,
}

#[derive(Debug, Clone)]
struct VariableDecl {
    name: String,
    ty: Type, // type but rust already uses that
    value: Assign,
}

// how functions are handled

#[derive(Debug, Clone)]
struct FuncDecl {
    func_id: u32,
    name: String,
    params: Vec<Param>,
    return_type: ReturnType,
    body: Block,
}

#[derive(Debug, Clone)]
struct FuncCall {
    func_id: u32,
    params: Vec<Param>,
}

#[derive(Debug, Clone)]
struct Param {
    pub name: String,
    pub type_: String,
    pub default: Option<ConstValue>,
}

// TODO how style is handled

// how rest is handled

#[derive(Debug, Clone)]
enum TextElement {
    Paragraph(String),
    Heading(String),
    List(Vec<TextElement>),
    Table(Vec<Vec<TextElement>>),
    Code(String, String), // Language, actual code
}

#[derive(Debug, Clone)]
enum HIRNode {
    Expr(Expr),
    Text(TextElement),
    Call(FuncCall),
}

// how template, document and style sections are handled

#[derive(Debug, Clone)]
struct Block {
    nodes: Vec<HIRNode>,
}

#[derive(Debug, Clone)]
pub struct HLIRModule {
    defaults: HashMap<String, ConstValue>,
    globals: Vec<GlobalDecl>, // top-level variables
    functions: Vec<FuncDecl>,
}

pub fn lower(ast: &Ast) -> HLIRModule {
    let mut interp = HLIRPass { ast: ast.clone() };
    interp.lower()
}

struct HLIRPass {
    // Fields and methods for the Hir struct
    ast: Ast,
}

impl HLIRPass {
    // Methods for the Hlir struct

    fn lower(&mut self) -> HLIRModule {
        let mut hlirmodule = HLIRModule {
            defaults: HashMap::new(),
            globals: Vec::new(),
            functions: Vec::new(),
        };
        self.lowerTemplateBlock(&mut hlirmodule);
        self.lowerDocumentBlock(&mut hlirmodule);
        self.lowerStyleBlock(&mut hlirmodule);

        println!("{:?}", hlirmodule);

        hlirmodule
    }

    fn lowerTemplateBlock(&mut self, hlirmodlue: &mut HLIRModule) {
        // all global, default and function declarations
        // handle defaults and globals inside this function call since they are small

        if let Some(template) = &self.ast.template {
            let statements = template.statements.clone();
            for statement in &statements {
                match statement {
                    crate::ast::Statement::VarAssign { name, value } => {
                        match value {
                            crate::ast::Expression::StringLiteral(s) => {
                                hlirmodlue.globals.push(GlobalDecl {
                                    name: name.clone(),
                                    ty: Type::String,
                                    value: Assign::Var(VarValue::String(s.clone())),
                                });
                            }
                            crate::ast::Expression::Int(n) => {
                                hlirmodlue.globals.push(GlobalDecl {
                                    name: name.clone(),
                                    ty: Type::Int,
                                    value: Assign::Var(VarValue::Int(*n)),
                                });
                            }
                            crate::ast::Expression::Float(n) => {
                                hlirmodlue.globals.push(GlobalDecl {
                                    name: name.clone(),
                                    ty: Type::Float,
                                    value: Assign::Var(VarValue::Float(*n)),
                                });
                            }
                            _ => {
                                // TODO handle other types
                            }
                        }
                    }
                    crate::ast::Statement::DefaultSet { key, value } => {
                        match value {
                            crate::ast::Expression::StringLiteral(s) => {
                                hlirmodlue
                                    .defaults
                                    .insert(key.clone(), ConstValue::String(s.clone()));
                            }
                            crate::ast::Expression::Int(n) => {
                                hlirmodlue.defaults.insert(key.clone(), ConstValue::Int(*n));
                            }
                            _ => {
                                // TODO handle other types
                            }
                        }
                    }
                    crate::ast::Statement::ConstAssign { name, value } => {
                        match value {
                            crate::ast::Expression::StringLiteral(s) => {
                                hlirmodlue.globals.push(GlobalDecl {
                                    name: name.clone(),
                                    ty: Type::String,
                                    value: Assign::Const(ConstValue::String(s.clone())),
                                });
                            }
                            crate::ast::Expression::Int(n) => {
                                hlirmodlue.globals.push(GlobalDecl {
                                    name: name.clone(),
                                    ty: Type::Int,
                                    value: Assign::Const(ConstValue::Int(*n)),
                                });
                            }
                            _ => {
                                // TODO handle other types
                            }
                        }
                    }
                    crate::ast::Statement::FunctionDecl { name, params, body } => {
                        let func_id = hlirmodlue.functions.len();
                        let hlir_body = self.lowerFunctionDecl(body);
                        hlirmodlue.functions.push(FuncDecl {
                            func_id: TryInto::<u32>::try_into(func_id).unwrap(),
                            name: name.clone(),
                            params: Vec::new(),            // TODO lower params
                            return_type: ReturnType::Void, // TODO check return type before setting
                            body: hlir_body,               // TODO lower body
                        });
                    }
                    _ => {}
                }
            }
        }
    }

    fn lowerDocumentBlock(&mut self, hlirmodlue: &mut HLIRModule) {
        let mut nodes = Vec::new();
        if let Some(document) = &self.ast.document {
            let statements = document.statements.clone();
            for statement in &statements {
                match statement {
                    crate::ast::Statement::ConstAssign { name, value } => match value {
                        crate::ast::Expression::StringLiteral(s) => {
                            nodes.push(HIRNode::Expr(Expr::VariableDecl(VariableDecl {
                                name: name.clone(),
                                ty: Type::String,
                                value: Assign::Var(VarValue::String(s.clone())),
                            })));
                        }
                        crate::ast::Expression::Int(n) => {
                            nodes.push(HIRNode::Expr(Expr::VariableDecl(VariableDecl {
                                name: name.clone(),
                                ty: Type::String,
                                value: Assign::Var(VarValue::Int(*n)),
                            })));
                        }
                        crate::ast::Expression::Float(n) => {
                            nodes.push(HIRNode::Expr(Expr::VariableDecl(VariableDecl {
                                name: name.clone(),
                                ty: Type::String,
                                value: Assign::Var(VarValue::Float(*n)),
                            })));
                        }
                        _ => {}
                    },
                    crate::ast::Statement::FunctionCall {
                        // TODO this is really ugly, refac crazy match statements
                        name,
                        args,
                        attributes,
                    } => {
                        let mut params = Vec::new();
                        for arg in args {
                            match arg {
                                crate::ast::KeyValue { key, value } => match value {
                                    crate::ast::Expression::StringLiteral(s) => {
                                        params.push(Param {
                                            name: key.clone(),
                                            type_: "String".to_string(),
                                            default: Some(ConstValue::String(s.clone())),
                                        });
                                    }
                                    crate::ast::Expression::Int(n) => {
                                        params.push(Param {
                                            name: key.clone(),
                                            type_: "Number".to_string(),
                                            default: Some(ConstValue::Int(*n)),
                                        });
                                    }
                                    crate::ast::Expression::Float(n) => {
                                        params.push(Param {
                                            name: key.clone(),
                                            type_: "Number".to_string(),
                                            default: Some(ConstValue::Float(*n)),
                                        });
                                    }
                                    _ => {}
                                },
                            }
                        }
                        for attr in attributes {
                            match attr {
                                crate::ast::KeyValue { key, value } => match value {
                                    crate::ast::Expression::StringLiteral(s) => {
                                        params.push(Param {
                                            name: key.clone(),
                                            type_: "String".to_string(),
                                            default: Some(ConstValue::String(s.clone())),
                                        });
                                    }
                                    &crate::ast::Expression::Int(n) => {
                                        params.push(Param {
                                            name: key.clone(),
                                            type_: "Number".to_string(),
                                            default: Some(ConstValue::Int(n)),
                                        });
                                    }
                                    crate::ast::Expression::Float(n) => {
                                        params.push(Param {
                                            name: key.clone(),
                                            type_: "Float".to_string(),
                                            default: Some(ConstValue::Float(*n)),
                                        });
                                    }
                                    _ => {}
                                },
                            }
                        }
                        // find function id off of the name
                        let mut func_id = 0;
                        for func in &hlirmodlue.functions {
                            if func.name == *name {
                                func_id = func.func_id;
                                break;
                            }
                        }
                        nodes.push(HIRNode::Call(FuncCall {
                            func_id,
                            params: params,
                        }));
                    }
                    crate::ast::Statement::Paragraph { value } => match value {
                        crate::ast::Expression::StringLiteral(text) => {
                            nodes.push(HIRNode::Text(TextElement::Paragraph(text.clone())));
                        }
                        // TODO others
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
        let func_id = hlirmodlue.functions.len();
        hlirmodlue.functions.push(FuncDecl {
            func_id: TryInto::<u32>::try_into(func_id).unwrap(),
            name: "__document".to_string(),
            params: Vec::new(),
            return_type: ReturnType::Void,
            body: Block { nodes: nodes },
        });
    }

    fn lowerStyleBlock(&mut self, hlirmodlue: &mut HLIRModule) {
        // TODO all style calls
    }

    fn lowerFunctionDecl(&mut self, body: &Vec<crate::ast::Statement>) -> Block {
        let mut nodes = Vec::new();
        for stmt in body {
            match stmt {
                crate::ast::Statement::ConstAssign { name, value } => match value {
                    crate::ast::Expression::StringLiteral(s) => {
                        nodes.push(HIRNode::Expr(Expr::VariableDecl(VariableDecl {
                            name: name.clone(),
                            ty: Type::String,
                            value: Assign::Var(VarValue::String(s.clone())),
                        })));
                    }
                    crate::ast::Expression::Int(n) => {
                        nodes.push(HIRNode::Expr(Expr::VariableDecl(VariableDecl {
                            name: name.clone(),
                            ty: Type::Int,
                            value: Assign::Var(VarValue::Int(*n)),
                        })));
                    }
                    // TODO other types
                    _ => {}
                },
                crate::ast::Statement::Return(expr) => {
                    match expr {
                        crate::ast::Expression::StringLiteral(s) => {
                            nodes.push(HIRNode::Expr(Expr::Return(VarValue::String(s.clone()))));
                        }
                        crate::ast::Expression::Int(n) => {
                            nodes.push(HIRNode::Expr(Expr::Return(VarValue::Int(*n))));
                        }
                        crate::ast::Expression::Float(n) => {
                            nodes.push(HIRNode::Expr(Expr::Return(VarValue::Float(*n))));
                        }
                        // TODO other types
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        Block { nodes }
    }
}
