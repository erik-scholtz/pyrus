use std::iter::Map;

use crate::ast::Ast;

pub enum ConstValue {
    Number(f64),
    Color(String),
    Str(String),
    Bool(bool),
}

pub struct FuncDecl {
    name: String,
    params: Vec<String>,
    body: Vec<()>, // Placeholder for function body statements
}

pub struct DocumentDecl {
    title: String,
    body: Vec<()>, // Placeholder for document body elements
}

pub struct StyleDecl {
    rules: Vec<()>, // Placeholder for style rules
}

pub struct GlobalDecl {
    name: String,
    value: ConstValue,
}

pub struct HLIRModule {
    defaults: Map<String, ConstValue>,
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
        let hlirmodlue = {};
        self.lowerTemplateBlock();
        self.lowerDocumentBlock();
        self.lowerStyleBlock();
    }

    fn lowerTemplateBlock(&mut self) {
        // all global, default and function declarations

    }

    fn lowerDocumentBlock(&mut self) {
        // TODO all function calls and default structure/document primatives calls
    }

    fn lowerStyleBlock(&mut self) {
        // TODO all style calls
    }

    fn lowerFunctionDecl(&mut self) {

    }

    fn lowerExpressionToTemp(&mut self) {

    }
}