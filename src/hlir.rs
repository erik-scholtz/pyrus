use std::collections::HashMap;
use std::env::args;

use crate::ast::{Ast, FuncAttributes, KeyValue};
use crate::types::{Literal, Type};

// IDs

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FuncId(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GlobalId(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValueId(pub u32);

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
        func: FuncId,
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
    let mut pass = HLIRPass {
        ast: ast.clone(),
        symbol_table: Vec::new(),
    };
    pass.lower()
}

struct HLIRPass {
    // Fields and methods for the Hir struct
    ast: Ast,
    symbol_table: Vec<HashMap<String, Id>>, // Scope stack
}

impl HLIRPass {
    // Methods for the Hlir struct

    fn lower(&mut self) -> HLIRModule {
        let mut hlirmodule = HLIRModule {
            globals: HashMap::new(),
            functions: HashMap::new(),
        };

        self.symbol_table.push(HashMap::new()); // add new scope (global)

        self.lower_template_block(&mut hlirmodule);
        self.lower_document_block(&mut hlirmodule);

        self.symbol_table.pop(); // remove scope (global)

        hlirmodule
    }

    fn lower_template_block(&mut self, hlirmodule: &mut HLIRModule) {
        // TODO clean this up more using the let something = match {} pattern
        // all global, default and function declarations
        // handle defaults and globals inside this function call since they are small
        let scope_index = self.symbol_table.len() - 1;

        if let Some(template) = &self.ast.template {
            let statements = template.statements.clone();
            for statement in &statements {
                match statement {
                    crate::ast::Statement::DefaultSet { key, value } => {
                        let global_id =
                            GlobalId(TryInto::<u32>::try_into(hlirmodule.globals.len()).unwrap());
                        let global = self.assign_global(
                            "__".to_string() + &key.clone(),
                            value.clone(),
                            global_id,
                        ); // TODO see if I can get rid of clone
                        hlirmodule.globals.insert(global_id, global);
                        self.add_symbol(key.clone(), Id::Global(global_id));
                    }
                    crate::ast::Statement::ConstAssign { name, value } => {
                        let global_id =
                            GlobalId(TryInto::<u32>::try_into(hlirmodule.globals.len()).unwrap());
                        let global = self.assign_global(name.clone(), value.clone(), global_id); // TODO see if I can get rid of clone
                        hlirmodule.globals.insert(global_id, global);
                        self.add_symbol(name.clone(), Id::Global(global_id));
                    }
                    crate::ast::Statement::VarAssign { name, value } => {
                        let global_id =
                            GlobalId(TryInto::<u32>::try_into(hlirmodule.globals.len()).unwrap());
                        let global = self.assign_global(name.clone(), value.clone(), global_id); // TODO see if I can get rid of clone
                        hlirmodule.globals.insert(global_id, global);
                        self.add_symbol(name.clone(), Id::Global(global_id));
                    }
                    crate::ast::Statement::FunctionDecl {
                        name,
                        args,
                        attributes,
                        body,
                    } => {
                        let func_id =
                            FuncId(TryInto::<u32>::try_into(hlirmodule.functions.len()).unwrap());
                        let hlir_body = self.lower_function_block(body);
                        self.add_symbol(name.clone(), Id::Func(func_id)); // adds function name to symbol table
                        let mut arg_list = Vec::new();
                        for arg in args {
                            match arg.ty.as_str() {
                                "Int" => arg_list.push(Type::Int),
                                "Float" => arg_list.push(Type::Float),
                                "String" => arg_list.push(Type::String),
                                _ => panic!("type not known"),
                            }
                        }

                        hlirmodule.functions.insert(
                            func_id,
                            Func {
                                id: func_id,
                                name: name.clone(),
                                args: arg_list,
                                attributes: attributes.clone(),
                                return_type: None, // TODO check return type before setting
                                body: hlir_body,
                            },
                        );
                    }
                    _ => {}
                }
            }
        }
    }

    fn lower_function_block(&mut self, body: &Vec<crate::ast::Statement>) -> Block {
        let mut ir_body = Block {
            ops: Vec::new(),
            text: Vec::new(),
        };

        self.symbol_table.push(HashMap::new()); // add new scope (function)
        let scope_index = self.symbol_table.len() - 1;

        for stmt in body {
            match stmt {
                crate::ast::Statement::ConstAssign { name, value } => {
                    let id = ValueId(TryInto::<u32>::try_into(ir_body.ops.len()).unwrap());
                    let value = self.assign_local(name.clone(), value.clone(), id);
                    ir_body.ops.push(value);
                    self.add_symbol(name.clone(), Id::Value(id));
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

        self.symbol_table.pop(); // remove scope (function)
        ir_body
    }

    fn lower_document_block(&mut self, hlirmodlue: &mut HLIRModule) {
        let mut ir_body = Block {
            ops: Vec::new(),
            text: Vec::new(),
        };

        self.symbol_table.push(HashMap::new()); // add new scope (document)
        let scope_index = self.symbol_table.len() - 1;

        if let Some(document) = &self.ast.document {
            let statements = document.statements.clone();
            for statement in &statements {
                match statement {
                    crate::ast::Statement::ConstAssign { name, value } => {
                        let id = ValueId(TryInto::<u32>::try_into(ir_body.ops.len()).unwrap());
                        let var = self.assign_local(name.clone(), value.clone(), id);
                        ir_body.ops.push(var);
                    }
                    crate::ast::Statement::FunctionCall {
                        // TODO this is really ugly, refac crazy match statements
                        name,
                        args,
                        attributes,
                    } => {
                        let mut args = self.handle_args(args, name);
                        let mut attributes = self.handle_attributes(attributes, name);

                        // find function id of of the name
                        let mut func_id = match self.find_symbol(name) {
                            Some(id) => Some(id),
                            None => panic!("Function not found: {}", name),
                        };

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
                args: Vec::new(),
                attributes: FuncAttributes::default(),
                return_type: None,
                body: ir_body,
            },
        );

        self.symbol_table.pop(); // remove scope (document)
    }

    // all var assigns in document and functions flow here
    fn assign_local(&mut self, name: String, value: crate::ast::Expression, id: ValueId) -> Op {
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

        // add variable to symbol table
        let len = self.symbol_table.len();
        let scope = self.symbol_table.get_mut(len - 1).unwrap(); // most recent scope
        scope.insert(name.clone(), Id::Value(id)); // add to known symbols

        op
    }

    // all vars from template and defaults flow in here
    fn assign_global(
        &mut self,
        name: String,
        value: crate::ast::Expression,
        id: GlobalId,
    ) -> Global {
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

    fn add_symbol(&mut self, name: String, id: Id) {
        for scope in self.symbol_table.iter_mut().rev() {
            if let Some(symbol) = scope.get(&name) {
                // TODO check if the the ids match, if there is a function defined with the same name as a variable then it should be ok or vice versa
                panic!("Symbol {} already exists", name);
            }
        }
        let len = self.symbol_table.len();
        let scope = self.symbol_table.get_mut(len - 1).unwrap(); // most recent scope
        scope.insert(name.clone(), id); // add to known symbols
    }

    fn find_symbol(&mut self, name: &str) -> Option<Id> {
        for scope in self.symbol_table.iter_mut().rev() {
            if let Some(symbol) = scope.get(name) {
                return Some(*symbol);
            }
        }
        None
    }

    fn handle_args(&mut self, arguments: &Vec<KeyValue>, name: &str) -> Vec<ValueId> {
        let mut args = Vec::new();
        for arg in arguments {
            // TODO this is really really bad will probably need to rethink a lot
            // TODO make an internal table for refering what vasr name is to know what var is being used/called on
            let func = match self.find_symbol(name) {
                Some(id) => id,
                None => panic!("Function not found"),
            };
            match arg {
                crate::ast::KeyValue { key, value } => {
                    for table in self.symbol_table.iter_mut().rev() {
                        if let Some(symbol) = table.get(key) {
                            match symbol {
                                Id::Value(id) => args.push(*id),
                                _ => {}
                            }
                            break;
                        } else {
                            // TODO handle cases where raw arguments are passed in
                            todo!("handle cases of raw arguments")
                        }
                    }
                }
            }
        }
        args
    }

    fn handle_attributes(&mut self, attributes: &Vec<KeyValue>, name: &str) -> FuncAttributes {
        let mut attrs = FuncAttributes::default();
        for attr in attributes {
            // TODO this is really really bad will probably need to rethink a lot
            // TODO make an internal table for refering what vasr name is to know what var is being used/called on
            match attr {
                crate::ast::KeyValue { key, value } => {}
            }
        }
        attrs
    }
}
