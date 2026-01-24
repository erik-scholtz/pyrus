use std::collections::HashMap;

// TODO each element should have a style attribute ID, these IDs are saved
// in a tree to show where they get overwritten by newer rules, solves
// the "every object has attributes associated with it" and the
// "how do I keep track of which attributes are inherited from parent elements"
//
// also could solve the HLIR doesnt need style attributes problem
// this is for IR step

use crate::ast::{Ast, DocElement, Statement};
use crate::hlir::ir_types::{
    AttributeNode, AttributeTree, Func, FuncBlock, FuncId, GlobalId, HLIRModule, Id, Op, Type,
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
            attributes: AttributeTree::new(),
            elements: Vec::new(),
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
                    Statement::DefaultSet { key, value } => {
                        let global_id =
                            GlobalId(TryInto::<usize>::try_into(hlirmodule.globals.len()).unwrap());
                        let global = self.assign_global(
                            "__".to_string() + &key.clone(),
                            value.clone(),
                            global_id,
                        ); // TODO see if I can get rid of clone
                        hlirmodule.globals.insert(global_id, global);
                        self.add_symbol(key.clone(), Id::Global(global_id));
                    }
                    Statement::ConstAssign { name, value } => {
                        let global_id =
                            GlobalId(TryInto::<usize>::try_into(hlirmodule.globals.len()).unwrap());
                        let global = self.assign_global(name.clone(), value.clone(), global_id); // TODO see if I can get rid of clone
                        hlirmodule.globals.insert(global_id, global);
                        self.add_symbol(name.clone(), Id::Global(global_id));
                    }
                    Statement::VarAssign { name, value } => {
                        let global_id =
                            GlobalId(TryInto::<usize>::try_into(hlirmodule.globals.len()).unwrap());
                        let global = self.assign_global(name.clone(), value.clone(), global_id); // TODO see if I can get rid of clone
                        hlirmodule.globals.insert(global_id, global);
                        self.add_symbol(name.clone(), Id::Global(global_id));
                    }
                    Statement::FunctionDecl { name, args, body } => {
                        let func_id =
                            FuncId(TryInto::<usize>::try_into(hlirmodule.functions.len()).unwrap());
                        let hlir_body = self.lower_function_block(body, hlirmodule);
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

    fn lower_document_block(&mut self, hlirmodule: &mut HLIRModule) {
        // TODO: redo this comepletely

        let mut ir_body = FuncBlock {
            ops: Vec::new(),
            returned_element_ref: 0,
        };

        self.symbol_table.push(HashMap::new()); // add new scope (document)

        if let Some(document) = &self.ast.document {
            let elements = document.elements.clone();
            for element in &elements {
                match element {
                    crate::ast::DocElement::Call { name, args } => {
                        // find function id of of the name
                        let func_id = match self.find_symbol(name.as_str()) {
                            Some(id) => Some(id),
                            None => panic!("Function not found: {}", name),
                        };

                        let args = self.handle_args(&args, &mut ir_body);

                        ir_body.ops.push(Op::Call {
                            // TODO fix id, func_id is found but is wrong type
                            func: func_id.unwrap(),
                            result: None,
                            args: args,
                        });
                    }
                    crate::ast::DocElement::Text {
                        content,
                        attributes,
                    } => {
                        hlirmodule.elements.push(DocElement::Text {
                            content: content.to_string(),
                            attributes: attributes.clone(),
                        });
                        let attribute_node = AttributeNode::new_with_attributes(
                            attributes,
                            hlirmodule.attributes.size,
                        );
                        let attributes_ref = hlirmodule.attributes.add_attribute(attribute_node);
                        let index = hlirmodule.elements.len() - 1;
                        ir_body.ops.push(Op::DocElementEmit {
                            index,
                            attributes_ref,
                        });
                        // TODO others, detext list, code snippets, images, links, etc
                    }
                    _ => {}
                }
            }
        }
        let func_id = FuncId(TryInto::<usize>::try_into(hlirmodule.functions.len()).unwrap());
        hlirmodule.functions.insert(
            func_id,
            Func {
                id: func_id,
                name: "__document".to_string(),
                args: Vec::new(),
                return_type: None,
                body: ir_body,
            },
        );

        self.symbol_table.pop(); // remove scope (document)
    }

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
