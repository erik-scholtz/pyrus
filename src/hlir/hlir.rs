use std::collections::HashMap;

use crate::ast::{ArgType, Ast, FuncAttributes, KeyValue};
use crate::hlir::ir_types::{
    Block, Func, FuncId, Global, GlobalId, HLIRModule, Id, Literal, Op, TextElement, Type, ValueId,
};

pub fn lower(ast: &Ast) -> HLIRModule {
    let mut pass = HLIRPass {
        ast: ast.clone(),
        symbol_table: Vec::new(),
    };
    pass.lower()
}

pub struct HLIRPass {
    // Fields and methods for the Hir struct
    ast: Ast,
    pub symbol_table: Vec<HashMap<String, Id>>, // Scope stack
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

    fn lower_document_block(&mut self, hlirmodlue: &mut HLIRModule) {
        let mut ir_body = Block {
            ops: Vec::new(),
            text: Vec::new(),
        };

        self.symbol_table.push(HashMap::new()); // add new scope (document)

        if let Some(document) = &self.ast.document {
            let statements = document.statements.clone();
            for statement in &statements {
                match statement {
                    crate::ast::Statement::ConstAssign { name, value } => {
                        // TODO, differnciate local and globakl functions
                        let id = ValueId(TryInto::<u32>::try_into(ir_body.ops.len()).unwrap());
                        let var = self.assign_local(name.clone(), value.clone(), id);
                        ir_body.ops.push(var);
                    }
                    crate::ast::Statement::VarAssign { name, value } => {
                        // TODO differnciate local and globakl functions
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
                        // find function id of of the name
                        let func_id = match self.find_symbol(name) {
                            Some(id) => Some(id),
                            None => panic!("Function not found: {}", name),
                        };

                        let args = self.handle_args(args, &mut ir_body);
                        let attributes = self.handle_attributes(attributes, name);

                        ir_body.ops.push(Op::Call {
                            // TODO fix id, func_id is found but is wrong type
                            func: func_id.unwrap(),
                            result: None,
                            args: args,
                            attributes: attributes,
                        });
                    }
                    crate::ast::Statement::Paragraph { value } => match value {
                        crate::ast::Expression::StringLiteral(text) => {
                            ir_body.text.push(TextElement::Paragraph(text.clone()));
                            let index = ir_body.text.len() - 1;
                            ir_body.ops.push(Op::TextRef { index });
                        }
                        // TODO others, detext list, code snippets, images, links, etc
                        _ => {}
                    },
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

    // all vars from template and defaults flow in here

    pub fn add_symbol(&mut self, name: String, id: Id) {
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
}
