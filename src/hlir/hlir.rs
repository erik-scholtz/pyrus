use std::collections::HashMap;

use crate::ast::{Ast, DocElement, Expression, Statement};
use crate::hlir::ir_types::{
    AttributeNode, AttributeTree, ElementMetadata, Func, FuncBlock, FuncId, GlobalId, HLIRModule,
    Id, Op, Type,
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
            css_rules: Vec::new(),
            elements: Vec::new(),
            element_metadata: Vec::new(),
        };

        self.symbol_table.push(HashMap::new()); // add new scope (global)

        self.lower_template_block(&mut hlirmodule);

        // Store CSS rules from AST
        if let Some(style) = &self.ast.style {
            hlirmodule.css_rules = style.statements.clone();
        }

        self.lower_document_block(&mut hlirmodule);

        self.symbol_table.pop(); // remove scope (global)

        hlirmodule
    }

    fn lower_template_block(&mut self, hlirmodule: &mut HLIRModule) {
        // all global, default and function declarations
        // handle defaults and globals inside this function call since they are small
        let _scope_index = self.symbol_table.len() - 1;

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
                            Id::Global(global_id),
                        ); // TODO see if I can get rid of clone
                        hlirmodule.globals.insert(Id::Global(global_id), global);
                        self.add_symbol(key.clone(), Id::Global(global_id));
                    }
                    Statement::ConstAssign { name, value } => {
                        let global_id =
                            GlobalId(TryInto::<usize>::try_into(hlirmodule.globals.len()).unwrap());
                        let global =
                            self.assign_global(name.clone(), value.clone(), Id::Global(global_id)); // TODO see if I can get rid of clone
                        hlirmodule.globals.insert(Id::Global(global_id), global);
                        self.add_symbol(name.clone(), Id::Global(global_id));
                    }
                    Statement::VarAssign { name, value } => {
                        let global_id =
                            GlobalId(TryInto::<usize>::try_into(hlirmodule.globals.len()).unwrap());
                        let global =
                            self.assign_global(name.clone(), value.clone(), Id::Global(global_id)); // TODO see if I can get rid of clone
                        hlirmodule.globals.insert(Id::Global(global_id), global);
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
                            Id::Func(func_id),
                            Func {
                                id: Id::Func(func_id),
                                name: name.clone(),
                                args: arg_list,
                                return_type: Some(Type::DocElement), // TODO check return type before setting (right now only DocElement)
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
        let mut ir_body = FuncBlock {
            ops: Vec::new(),
            returned_element_ref: Some(0), // TODO this return type, magic number and I have a feeling that its wrong
        };

        self.symbol_table.push(HashMap::new()); // add new scope (document)

        if let Some(document) = &self.ast.document {
            let elements = document.elements.clone();
            for element in &elements {
                self.lower_document_element(
                    element,
                    hlirmodule,
                    &mut ir_body,
                    None, // No parent for top-level elements
                );
            }
        }
        let func_id = FuncId(TryInto::<usize>::try_into(hlirmodule.functions.len()).unwrap());
        hlirmodule.functions.insert(
            Id::Func(func_id),
            Func {
                id: Id::Func(func_id),
                name: "__document".to_string(),
                args: Vec::new(),
                return_type: Some(Type::DocElement), // For right now only DocElements are supported TODO add in other types support later
                body: ir_body,
            },
        );

        self.symbol_table.pop(); // remove scope (document)
    }

    fn lower_document_element(
        &mut self,
        element: &crate::ast::DocElement,
        hlirmodule: &mut HLIRModule,
        ir_body: &mut FuncBlock,
        parent_index: Option<usize>,
    ) {
        match element {
            crate::ast::DocElement::Call { name, args } => {
                let func_id = match self.find_symbol(name.as_str()) {
                    Some(id) => Some(id),
                    None => panic!("Function not found: {}", name),
                };

                let arg_value_ids = self.handle_args(args, ir_body);
                ir_body.ops.push(Op::Call {
                    func: func_id.unwrap(),
                    result: None,
                    args: arg_value_ids,
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

                let (id, classes) = self.extract_id_and_classes(attributes);
                let element_type = "text".to_string();
                let attribute_node =
                    AttributeNode::new_with_attributes(attributes, hlirmodule.attributes.size);
                let attributes_ref = hlirmodule.attributes.add_attribute(attribute_node);
                hlirmodule.element_metadata.push(ElementMetadata {
                    id,
                    classes,
                    element_type,
                    parent: parent_index,
                    attributes_ref,
                });

                let index = hlirmodule.elements.len() - 1;
                ir_body.ops.push(Op::DocElementEmit {
                    index,
                    attributes_ref,
                });
            }
            crate::ast::DocElement::Section {
                elements: section_elements,
                attributes,
            } => {
                hlirmodule.elements.push(DocElement::Section {
                    elements: section_elements.clone(),
                    attributes: attributes.clone(),
                });

                let (id, classes) = self.extract_id_and_classes(attributes);
                let element_type = "section".to_string();
                let attribute_node =
                    AttributeNode::new_with_attributes(attributes, hlirmodule.attributes.size);
                let attributes_ref = hlirmodule.attributes.add_attribute(attribute_node);
                hlirmodule.element_metadata.push(ElementMetadata {
                    id,
                    classes,
                    element_type,
                    parent: parent_index,
                    attributes_ref,
                });

                let index = hlirmodule.elements.len() - 1;
                ir_body.ops.push(Op::DocElementEmit {
                    index,
                    attributes_ref,
                });

                // Recursively lower child elements with this section as parent
                for child in section_elements {
                    self.lower_document_element(child, hlirmodule, ir_body, Some(index));
                }
            }
            crate::ast::DocElement::List { items, attributes } => {
                hlirmodule.elements.push(DocElement::List {
                    items: items.clone(),
                    attributes: attributes.clone(),
                });

                let (id, classes) = self.extract_id_and_classes(attributes);
                let element_type = "list".to_string();
                let attribute_node =
                    AttributeNode::new_with_attributes(attributes, hlirmodule.attributes.size);
                let attributes_ref = hlirmodule.attributes.add_attribute(attribute_node);
                hlirmodule.element_metadata.push(ElementMetadata {
                    id,
                    classes,
                    element_type,
                    parent: parent_index,
                    attributes_ref,
                });

                let index = hlirmodule.elements.len() - 1;
                ir_body.ops.push(Op::DocElementEmit {
                    index,
                    attributes_ref,
                });

                // Recursively lower list items with this list as parent
                for child in items {
                    self.lower_document_element(child, hlirmodule, ir_body, Some(index));
                }
            }
            // TODO: Handle Image, Code, Link, Table similarly
            _ => {}
        }
    }

    /// Extract id and classes from element attributes
    fn extract_id_and_classes(
        &self,
        attributes: &HashMap<String, Expression>,
    ) -> (Option<String>, Vec<String>) {
        let id = attributes.get("id").map(|e| e.to_string());

        let classes = attributes
            .get("class")
            .map(|e| e.to_string().split_whitespace().map(String::from).collect())
            .unwrap_or_default();

        (id, classes)
    }

    pub fn add_symbol(&mut self, name: String, id: Id) {
        for scope in self.symbol_table.iter_mut().rev() {
            if let Some(_symbol) = scope.get(&name) {
                // TODO check if the the id types match (Func/value/global), if there is a function defined with the same name as a variable then it should be ok or vice versa

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
