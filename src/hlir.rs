use std::collections::HashMap;

use crate::ast::Ast;
use crate::types::{Literal, Type};

// IDs

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct FuncId(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GlobalId(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValueId(pub u32);

// default variables like the authur or default font size

pub enum Value {
    Literal(Literal),
    Ref(ValueId),
}

// rest of values expressed as expressions, I think this makes sense for now

#[derive(Debug, Clone)]
pub enum Op {
    Const {
        result: ValueId,
        literal: Literal,
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
        func: FuncId,
        args: Vec<ValueId>, // see if this needs to be a CallArg struct
        attributes: Vec<FuncAttribute>,
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
    pub params: Vec<Type>,
    pub attributes: Vec<FuncAttribute>,
    pub return_type: Option<Type>,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct FuncAttribute {
    pub name: String,
    pub value: Literal,
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
    globals: HashMap<GlobalId, Global>, // TODO eventually remove IDs from actual struct and just refer to them (I think)
    functions: HashMap<FuncId, Func>,
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
            globals: HashMap::new(),
            functions: HashMap::new(),
        };
        self.lowerTemplateBlock(&mut hlirmodule);
        self.lowerDocumentBlock(&mut hlirmodule);

        println!("{:?}", hlirmodule);

        hlirmodule
    }

    fn lowerTemplateBlock(&mut self, hlirmodule: &mut HLIRModule) {
        // TODO clean this up more using the let something = match {} pattern
        // all global, default and function declarations
        // handle defaults and globals inside this function call since they are small

        if let Some(template) = &self.ast.template {
            let statements = template.statements.clone();
            for statement in &statements {
                match statement {
                    crate::ast::Statement::DefaultSet { key, value } => {
                        let global_id =
                            GlobalId(TryInto::<u32>::try_into(hlirmodule.globals.len()).unwrap());
                        let global = assign_global(
                            "__".to_string() + &key.clone(),
                            value.clone(),
                            global_id,
                        ); // TODO see if I can get rid of clone
                        hlirmodule.globals.insert(global_id, global);
                    }
                    crate::ast::Statement::ConstAssign { name, value } => {
                        let global_id =
                            GlobalId(TryInto::<u32>::try_into(hlirmodule.globals.len()).unwrap());
                        let global = assign_global(name.clone(), value.clone(), global_id); // TODO see if I can get rid of clone
                        hlirmodule.globals.insert(global_id, global);
                    }
                    crate::ast::Statement::VarAssign { name, value } => {
                        let global_id =
                            GlobalId(TryInto::<u32>::try_into(hlirmodule.globals.len()).unwrap());
                        let global = assign_global(name.clone(), value.clone(), global_id); // TODO see if I can get rid of clone
                        hlirmodule.globals.insert(global_id, global);
                    }
                    crate::ast::Statement::FunctionDecl {
                        name,
                        params,
                        attributes,
                        body,
                    } => {
                        let func_id =
                            FuncId(TryInto::<u32>::try_into(hlirmodule.functions.len()).unwrap());
                        let hlir_body = self.lowerFunctionBlock(body);
                        hlirmodule.functions.insert(
                            func_id,
                            Func {
                                id: func_id,
                                name: name.clone(),
                                params: Vec::new(),     // TODO lower params
                                attributes: Vec::new(), // TODO lower attributes
                                return_type: None,      // TODO check return type before setting
                                body: hlir_body,
                            },
                        );
                    }
                    _ => {}
                }
            }
        }
    }

    fn lowerFunctionBlock(&mut self, body: &Vec<crate::ast::Statement>) -> Block {
        let mut ir_body = Block {
            ops: Vec::new(),
            text: Vec::new(),
        };

        for stmt in body {
            match stmt {
                crate::ast::Statement::ConstAssign { name, value } => {
                    let id = ValueId(TryInto::<u32>::try_into(ir_body.ops.len()).unwrap());
                    let value = assign_local(name.clone(), value.clone(), id);
                    ir_body.ops.push(value);
                }
                crate::ast::Statement::Return(expr) => {
                    let value_id = ValueId(TryInto::<u32>::try_into(ir_body.ops.len()).unwrap());
                    ir_body.ops.push(Op::Const {
                        result: value_id,
                        literal: Literal::Int(0),
                        ty: Type::Int,
                    });
                    ir_body.ops.push(Op::Return {
                        value: Some(value_id),
                    });
                }
                _ => {
                    todo!("other types not handled yet")
                }
            }
        }
        ir_body
    }

    fn lowerDocumentBlock(&mut self, hlirmodlue: &mut HLIRModule) {
        let mut ir_body = Block {
            ops: Vec::new(),
            text: Vec::new(),
        };

        if let Some(document) = &self.ast.document {
            let statements = document.statements.clone();
            for statement in &statements {
                match statement {
                    crate::ast::Statement::ConstAssign { name, value } => {
                        let id = ValueId(TryInto::<u32>::try_into(ir_body.ops.len()).unwrap());
                        let var = assign_local(name.clone(), value.clone(), id);
                        ir_body.ops.push(var);
                    }
                    crate::ast::Statement::FunctionCall {
                        // TODO this is really ugly, refac crazy match statements
                        name,
                        args,
                        attributes,
                    } => {
                        let mut args = Vec::new();
                        let mut attributes = Vec::new();
                        // for arg in args { // TODO this is really really bad will probably need to rethink a lot
                        //     match arg {
                        //         crate::ast::KeyValue { key, value } => {
                        //             for op in ir_body.ops {
                        //                 match op {
                        //                     Op::Const {
                        //                         result,
                        //                         literal,
                        //                         ty,
                        //                     } => {
                        //                         if result == *key {
                        //                             params.push(HIRNode::Const(Const {
                        //                                 value: literal.clone(),
                        //                                 ty: ty.clone(),
                        //                             }));
                        //                         }
                        //                     }
                        //                     _ => {}
                        //                 }
                        //             }
                        //         }
                        //     }
                        // }
                        // for attr in attributes {
                        //     match attr {}
                        // }
                        // find function id off of the name
                        // let mut func_id = 0;
                        // for func in &hlirmodlue.functions {
                        //     if func.name == *name {
                        //         func_id = func.id;
                        //         break;
                        //     }
                        // }
                        ir_body.ops.push(Op::Call {
                            // TODO
                            func: FuncId(0),
                            result: None,
                            args: args,
                            attributes: attributes,
                        });
                    }
                    // crate::ast::Statement::Paragraph { value } => match value {
                    //     crate::ast::Expression::StringLiteral(text) => {
                    //         nodes.push(HIRNode::Text(TextElement::Paragraph(text.clone())));
                    //     }
                    //     // TODO others
                    //     _ => {}
                    // },
                    _ => {}
                }
            }
        }
        let func_id = FuncId(TryInto::<u32>::try_into(hlirmodlue.functions.len()).unwrap());
        hlirmodlue.functions.insert(
            func_id,
            Func {
                id: func_id,
                name: "__document".to_string(),
                params: Vec::new(),
                attributes: Vec::new(),
                return_type: None,
                body: ir_body,
            },
        );
    }
}

fn handle_params() -> Vec<Type> {
    todo!("Implement parameter handling");
}
// all var assigns in document and functions flow here
fn assign_local(name: String, value: crate::ast::Expression, id: ValueId) -> Op {
    let op = match value {
        crate::ast::Expression::StringLiteral(s) => Op::Const {
            result: id,
            literal: Literal::String(s.clone()),
            ty: Type::String,
        },
        crate::ast::Expression::Int(n) => Op::Const {
            result: id,
            literal: Literal::Int(n),
            ty: Type::Int,
        },
        crate::ast::Expression::Float(n) => Op::Const {
            result: id,
            literal: Literal::Float(n),
            ty: Type::Float,
        },
        _ => {
            todo!("implement other expression types")
        }
    };
    op
}

// all vars from template and defaults flow in here
fn assign_global(name: String, value: crate::ast::Expression, id: GlobalId) -> Global {
    let global = match value {
        crate::ast::Expression::StringLiteral(s) => Global {
            id: id,
            name: name.clone(),
            ty: Type::String,
            init: Literal::String(s.clone()),
        },
        crate::ast::Expression::Int(n) => Global {
            id: id,
            name: name.clone(),
            ty: Type::Int,
            init: Literal::Int(n),
        },
        crate::ast::Expression::Float(n) => Global {
            id: id,
            name: name.clone(),
            ty: Type::Float,
            init: Literal::Float(n),
        },
        _ => {
            todo!("implement other expression types")
        }
    };
    global
}
