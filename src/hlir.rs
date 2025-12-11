use std::collections::HashMap;

use crate::ast::Ast;

#[derive(Clone)]
pub enum ConstValue {
    Number(f64),
    Color(String),
    Str(String),
    Bool(bool),
}

#[derive(Clone)]
pub struct FuncDecl {
    name: String,
    params: Vec<String>,
    body: Vec<()>, // Placeholder for function body statements
}

impl FuncDecl {
    pub fn new(name: String, params: Vec<String>, body: Vec<()>) -> Self {
        FuncDecl { name, params, body }
    }
}

#[derive(Clone)]
pub struct DocumentDecl {
    title: String,
    body: Vec<()>, // Placeholder for document body elements
}

impl DocumentDecl {
    pub fn new() -> Self {
        DocumentDecl {
            title: String::new(),
            body: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct StyleDecl {
    rules: Vec<()>, // Placeholder for style rules
}

impl StyleDecl {
    pub fn new() -> Self {
        StyleDecl { rules: Vec::new() }
    }
}

#[derive(Clone)]
pub struct GlobalDecl {
    name: String,
    value: ConstValue,
}

#[derive(Clone)]
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
        let hlirmodlue = HLIRModule {
            defaults: HashMap::new(),
            globals: Vec::new(),
            functions: Vec::new(),
            document: DocumentDecl::new(),
            stylesheet: StyleDecl::new(),
        };
        self.lowerTemplateBlock(&hlirmodlue);
        self.lowerDocumentBlock(&hlirmodlue);
        self.lowerStyleBlock(&hlirmodlue);
    }

    fn lowerTemplateBlock(&mut self, hlirmodlue: &HLIRModule) {
        // all global, default and function declarations
        // handle defaults and globals inside this function call since they are small
        self.lowerFunctionDecl(hlirmodlue);
    }

    fn lowerDocumentBlock(&mut self, hlirmodlue: &HLIRModule) {
        // TODO all function calls and default structure/document primatives calls
    }

    fn lowerStyleBlock(&mut self, hlirmodlue: &HLIRModule) {
        // TODO all style calls
    }

    fn lowerFunctionDecl(&mut self, hlirmodlue: &HLIRModule) {

    }

    fn lowerExpressionToTemp(&mut self) {}
}
