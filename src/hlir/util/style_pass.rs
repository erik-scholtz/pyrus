use crate::hlir::hlir::HLIRPass;
use crate::hlir::util::style_resolver::resolve_styles;

impl HLIRPass {
    /// Run the CSS style resolution pass on the HLIR module
    pub fn style_pass(&mut self, hlir: &mut crate::hlir::ir_types::HLIRModule) {
        resolve_styles(hlir);
    }
}
