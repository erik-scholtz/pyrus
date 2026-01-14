use std::collections::HashMap;

use crate::hlir::hlir::HLIRPass;
use crate::hlir::ir_types::{Block, Id, Literal, Op, Type, ValueId};

use crate::ast::{ArgType, FuncAttributes, KeyValue};

impl HLIRPass {
    pub fn lower_function_block(&mut self, body: &Vec<crate::ast::Statement>) -> Block {
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

    pub fn handle_args(&mut self, arguments: &Vec<ArgType>, ir_body: &mut Block) -> Vec<ValueId> {
        self.symbol_table.push(HashMap::new()); // adding new table for arg scope
        let mut args = Vec::new();
        for crate::ast::ArgType { name, ty } in arguments {
            // TODO this is really really bad will probably need to rethink a lot
            // TODO make an internal table for refering what var name is to know what var is being used/called on

            // TODO handle cases where raw arguments are passed in
            // maybe look at instead of passing "arg" pass the variable type or
            // somethig if the var is not decalred, pass "var" if declared
            // for right now if there is a quotes or number, assume raw arg
            match ty.as_str() {
                "var" => {
                    for table in self.symbol_table.iter_mut().rev() {
                        if let Some(symbol) = table.get(name) {
                            match symbol {
                                Id::Value(id) => {
                                    args.push(*id);
                                }
                                _ => {}
                            }
                        }
                    }
                }
                "int" => {
                    let value = name.as_str().parse::<i64>().unwrap();
                    let id = ValueId(TryInto::<u32>::try_into(ir_body.ops.len()).unwrap());
                    let var_name = "raw_arg_".to_string() + id.to_string().as_str();
                    let var =
                        self.assign_local(var_name.clone(), crate::ast::Expression::Int(value), id);
                    ir_body.ops.push(var);
                    args.push(id);
                }
                "float" => {
                    let value = name.as_str().parse::<f64>().unwrap();
                    let id = ValueId(TryInto::<u32>::try_into(ir_body.ops.len()).unwrap());
                    let var_name = "raw_arg_".to_string() + id.to_string().as_str();
                    let var = self.assign_local(
                        var_name.clone(),
                        crate::ast::Expression::Float(value),
                        id,
                    );
                    ir_body.ops.push(var);
                    args.push(id);
                }
                "string" => {
                    let value = name.as_str().parse::<String>().unwrap();
                    let id = ValueId(TryInto::<u32>::try_into(ir_body.ops.len()).unwrap());
                    let var_name = "raw_arg_".to_string() + id.to_string().as_str();
                    let var = self.assign_local(
                        var_name.clone(),
                        crate::ast::Expression::StringLiteral(value),
                        id,
                    );
                    ir_body.ops.push(var);
                    args.push(id);
                }
                _ => {}
            }
        }
        self.symbol_table.pop();
        args
    }

    pub fn handle_attributes(&mut self, attributes: &Vec<KeyValue>, name: &str) -> FuncAttributes {
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
