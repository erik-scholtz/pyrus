use std::collections::{HashMap, btree_map::Values};

use crate::ast::{Ast, Statement};

#[derive(Debug, Clone)]
enum Op {
    Assign(String, ConstValue),
    Call(String, Vec<ConstValue>),
    If(ConstValue, Block, Option<Block>),
    While(ConstValue, Block),
    Return(ConstValue),
}

#[derive(Debug, Clone)]
pub struct Block {
    ops: Vec<Op>,
}

#[derive(Debug, Clone)]
pub enum ConstValue {
    Number(i64),
    Color(String),
    Str(String),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub struct FuncDecl {
    name: String,
    params: Vec<String>,
    body: Block, // Placeholder for function body statements
}

impl FuncDecl {
    pub fn new(name: String, params: Vec<String>, body: Block) -> Self {
        FuncDecl { name, params, body }
    }
}

#[derive(Debug, Clone)]
pub struct DocumentDecl {
    title: String,
    body: Block, // Placeholder for document body elements
}

impl DocumentDecl {
    pub fn new() -> Self {
        DocumentDecl {
            title: String::new(),
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
    }

    fn lowerTemplateBlock(&mut self, hlirmodlue: &mut HLIRModule) {
        // all global, default and function declarations
        // handle defaults and globals inside this function call since they are small

        // loop over ast
        if let Some(template) = &self.ast.template {
            let statements = template.statements.clone();
            for statement in &statements {
                match statement {
                    crate::ast::Statement::VarAssign { name, value: value } => {
                        match value {
                            crate::ast::Expression::StringLiteral(s) => {
                                hlirmodlue.globals.push(GlobalDecl {
                                    name: name.clone(),
                                    value: ConstValue::Str(s.clone()),
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
                                    .insert(key.clone(), ConstValue::Str(s.clone()));
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
                        // TODO handle constant declaration
                    }
                    crate::ast::Statement::FunctionDecl { name, params, body } => {
                        // self.lowerFunctionDecl(hlirmodlue);
                        let func_index = usize::try_from(self.freshTemp()).unwrap();
                        hlirmodlue.functions.insert(
                            func_index,
                            FuncDecl {
                                name: name.clone(),
                                params: Vec::new(), // TODO lower params
                                body: Block { ops: Vec::new() }, // TODO lower body
                            },
                        );
                    }
                    _ => {}
                }
            }
        }
        println!("HLIR Defaults: {:?}", hlirmodlue.defaults);
        println!("HLIR Globals: {:?}", hlirmodlue.globals);
    }

    fn lowerDocumentBlock(&mut self, hlirmodlue: &mut HLIRModule) {
        // TODO all function calls and default structure/document primatives calls
    }

    fn lowerStyleBlock(&mut self, hlirmodlue: &mut HLIRModule) {
        // TODO all style calls
    }

    fn lowerFunctionDecl(&mut self, hlirmodlue: &HLIRModule) {}

    fn lowerExpressionToTemp(&mut self) {}
}
