use std::collections::{HashMap, btree_map::Values};

use crate::ast::Ast;

#[derive(Debug, Clone)]
enum Op {
    ConstAssign(String, ConstValue),
    Assign(String, ConstValue),
    Call(String, Vec<ParamItem>),
    If(ConstValue, Block, Option<Block>),
    While(ConstValue, Block),
    Return(ConstValue),
    Paragraph(String), // TODO: remove and create Element::Paragraph
}

#[derive(Debug, Clone)]
struct ParamItem {
    name: String,
    type_: String,
    default: Option<ConstValue>,
}

// TODO
enum Element {
    Paragraph(String),
    Heading(String),
    List(Vec<Element>),
    Table(Vec<Vec<Element>>),
    Image(String, String),
    Code(String, String), // Language, actual code
}

#[derive(Debug, Clone)]
pub struct Block {
    ops: Vec<Op>,
}

#[derive(Debug, Clone)]
pub enum ConstValue {
    Number(i64),
    Color(String),
    String(String),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub type_: String,
    pub default: Option<ConstValue>,
}

#[derive(Debug, Clone)]
pub struct FuncDecl {
    name: String,
    params: Vec<Param>,
    body: Block, // Placeholder for function body statements
}

#[derive(Debug, Clone)]
pub struct DocumentDecl {
    body: Block, // Placeholder for document body elements
}

impl DocumentDecl {
    pub fn new() -> Self {
        DocumentDecl {
            body: Block { ops: Vec::new() },
        }
    }
}

#[derive(Debug, Clone)]
pub struct StyleDecl {
    rules: Vec<()>, // Placeholder for style rules
}

impl StyleDecl {
    pub fn new() -> Self {
        StyleDecl { rules: Vec::new() }
    }
}

#[derive(Debug, Clone)]
pub struct GlobalDecl {
    name: String,
    value: ConstValue,
}

#[derive(Debug, Clone)]
pub struct HLIRModule {
    defaults: HashMap<String, ConstValue>,
    globals: Vec<GlobalDecl>, // top-level variables
    functions: Vec<FuncDecl>,
    document: DocumentDecl,
    stylesheet: StyleDecl,
}

pub fn lower(ast: &Ast) {
    let mut interp = HlirInterp {
        fresh_temp: 0,
        ast: ast.clone(),
    };
    interp.lower();
}

struct HlirInterp {
    // Fields and methods for the Hlir struct
    fresh_temp: u32,
    ast: Ast,
}

impl HlirInterp {
    // Methods for the Hlir struct
    fn freshTemp(&mut self) -> u32 {
        let temp = self.fresh_temp;
        self.fresh_temp += 1;
        temp
    }

    fn lower(&mut self) {
        let mut hlirmodlue = HLIRModule {
            defaults: HashMap::new(),
            globals: Vec::new(),
            functions: Vec::new(),
            document: DocumentDecl::new(),
            stylesheet: StyleDecl::new(),
        };
        self.lowerTemplateBlock(&mut hlirmodlue);
        self.lowerDocumentBlock(&mut hlirmodlue);
        self.lowerStyleBlock(&mut hlirmodlue);

        println!("{:?}", hlirmodlue);
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
                                    value: ConstValue::String(s.clone()),
                                });
                            }
                            crate::ast::Expression::NumberLiteral(n) => {
                                hlirmodlue.globals.push(GlobalDecl {
                                    name: name.clone(),
                                    value: ConstValue::Number(*n),
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
                            crate::ast::Expression::NumberLiteral(n) => {
                                hlirmodlue
                                    .defaults
                                    .insert(key.clone(), ConstValue::Number(*n));
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
                                    value: ConstValue::String(s.clone()),
                                });
                            }
                            crate::ast::Expression::NumberLiteral(n) => {
                                hlirmodlue.globals.push(GlobalDecl {
                                    name: name.clone(),
                                    value: ConstValue::Number(*n),
                                });
                            }
                            _ => {
                                // TODO handle other types
                            }
                        }
                    }
                    crate::ast::Statement::FunctionDecl { name, params, body } => {
                        let func_index = usize::try_from(self.freshTemp()).unwrap();
                        let hlir_body = self.lowerFunctionDecl(body);
                        hlirmodlue.functions.insert(
                            func_index,
                            FuncDecl {
                                name: name.clone(),
                                params: Vec::new(), // TODO lower params
                                body: hlir_body,    // TODO lower body
                            },
                        );
                    }
                    _ => {}
                }
            }
        }
    }

    fn lowerDocumentBlock(&mut self, hlirmodlue: &mut HLIRModule) {
        let mut ops = Vec::new();
        if let Some(document) = &self.ast.document {
            let statements = document.statements.clone();
            for statement in &statements {
                match statement {
                    crate::ast::Statement::ConstAssign { name, value } => match value {
                        crate::ast::Expression::StringLiteral(s) => {
                            ops.push(Op::ConstAssign(name.clone(), ConstValue::String(s.clone())));
                        }
                        crate::ast::Expression::NumberLiteral(n) => {
                            ops.push(Op::ConstAssign(name.clone(), ConstValue::Number(*n)));
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
                                        params.push(ParamItem {
                                            name: key.clone(),
                                            type_: "String".to_string(),
                                            default: Some(ConstValue::String(s.clone())),
                                        });
                                    }
                                    crate::ast::Expression::NumberLiteral(n) => {
                                        params.push(ParamItem {
                                            name: key.clone(),
                                            type_: "Number".to_string(),
                                            default: Some(ConstValue::Number(*n)),
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
                                        params.push(ParamItem {
                                            name: key.clone(),
                                            type_: "String".to_string(),
                                            default: Some(ConstValue::String(s.clone())),
                                        });
                                    }
                                    &crate::ast::Expression::NumberLiteral(n) => {
                                        params.push(ParamItem {
                                            name: key.clone(),
                                            type_: "Number".to_string(),
                                            default: Some(ConstValue::Number(n)),
                                        });
                                    }
                                    _ => {}
                                },
                            }
                        }
                        ops.push(Op::Call(name.clone(), params));
                    }
                    crate::ast::Statement::Paragraph { value } => match value {
                        crate::ast::Expression::StringLiteral(text) => {
                            ops.push(Op::Paragraph(text.clone()));
                        }
                        // TODO others
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
        hlirmodlue.document = DocumentDecl {
            body: Block { ops },
        };
    }

    fn lowerStyleBlock(&mut self, hlirmodlue: &mut HLIRModule) {
        // TODO all style calls
    }

    fn lowerFunctionDecl(&mut self, body: &Vec<crate::ast::Statement>) -> Block {
        let mut ops = Vec::new();
        for stmt in body {
            match stmt {
                crate::ast::Statement::ConstAssign { name, value } => match value {
                    crate::ast::Expression::StringLiteral(s) => {
                        ops.push(Op::ConstAssign(name.clone(), ConstValue::String(s.clone())));
                    }
                    crate::ast::Expression::NumberLiteral(n) => {
                        ops.push(Op::ConstAssign(name.clone(), ConstValue::Number(n.clone())));
                    }
                    // TODO other types
                    _ => {}
                },
                crate::ast::Statement::Return(expr) => {
                    match expr {
                        crate::ast::Expression::StringLiteral(s) => {
                            ops.push(Op::ConstAssign(
                                "return_value".to_string(),
                                ConstValue::String(s.clone()),
                            ));
                        }
                        crate::ast::Expression::NumberLiteral(n) => {
                            ops.push(Op::ConstAssign(
                                "return_value".to_string(),
                                ConstValue::Number(n.clone()),
                            ));
                        }
                        // TODO other types
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        Block { ops }
    }
}
