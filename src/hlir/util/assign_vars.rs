use crate::hlir::hlir::HLIRPass;

use crate::hlir::ir_types::{Global, GlobalId, Id, Literal, Op, Type, ValueId};

impl HLIRPass {
    pub fn assign_global(
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

    pub fn assign_local(&mut self, name: String, value: crate::ast::Expression, id: ValueId) -> Op {
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
}
