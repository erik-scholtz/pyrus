use taffy::{NodeId, Style, TaffyTree};

use crate::hlir::{Func, FuncId, HLIRModule, Literal, Op, StyleAttributes, Type};

pub fn setup_layout(hlir_module: &HLIRModule) -> LayoutEngine {
    // Implement layout setup logic here
    let layout = LayoutEngine::build_from_hlir_module(hlir_module);

    layout
}

#[derive(Debug)]
pub struct LayoutEngine {
    tree: TaffyTree,
    root: NodeId,
}

impl LayoutEngine {
    pub fn new() -> Self {
        LayoutEngine {
            tree: TaffyTree::new(),
            root: NodeId::new(0),
        }
    }

    pub fn build_from_hlir_module(hlir_module: &HLIRModule) -> Self {
        let mut layout = LayoutEngine::new();
        let document_id = FuncId(hlir_module.functions.len() - 1);

        let document = hlir_module
            .functions
            .get(&document_id)
            .expect("document function not found")
            .clone(); // panics if None TODO in the future have a defualt document type be created

        layout.root = layout
            .tree
            .new_with_children(Style::default(), &[])
            .unwrap();

        for op in document.body.ops {
            match op {
                Op::DocElementEmit {
                    index,
                    attributes_ref,
                } => {
                    let element = layout.tree.new_leaf(Style::default());
                    match element {
                        Ok(element) => {
                            layout.tree.add_child(layout.root, element).unwrap();
                        }
                        Err(err) => {
                            panic!("Failed to create element: {}", err);
                        }
                    }
                }
                Op::Call { result, func, args } => {}
                _ => {}
            }
        }
        layout
    }
}
