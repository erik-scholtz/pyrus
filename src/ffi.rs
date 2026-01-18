use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_uchar};

#[repr(C)]
pub struct HLIR_Error {
    pub code: c_int,
    pub message: *const c_char,
}

#[repr(C)]
pub struct HLIRModule;

#[repr(C)]
pub struct DocModule;

unsafe extern "C" {
    pub fn hlir_lower_to_doc(module: *const HLIRModule, err: *mut HLIR_Error) -> *mut DocModule;

    pub fn doc_module_serialize(
        doc: *mut DocModule,
        size: *mut usize,
        err: *mut HLIR_Error,
    ) -> *mut c_uchar;

    pub fn doc_module_destroy(doc: *mut DocModule);

    pub fn hlir_free_buffer(buf: *mut c_uchar);
}

pub fn lower_to_doc(module: &HLIRModule) -> Result<Vec<u8>, String> {
    unsafe {
        let mut err = HLIR_Error {
            code: 0,
            message: std::ptr::null(),
        };

        let doc = hlir_lower_to_doc(module as *const HLIRModule, &mut err);

        if doc.is_null() {
            return Err(CStr::from_ptr(err.message).to_string_lossy().into_owned());
        }

        let mut size = 0;
        let buf = doc_module_serialize(doc, &mut size, &mut err);
        doc_module_destroy(doc);

        if buf.is_null() {
            return Err(CStr::from_ptr(err.message).to_string_lossy().into_owned());
        }

        let slice = std::slice::from_raw_parts(buf, size).to_vec();
        hlir_free_buffer(buf as _);

        Ok(slice)
    }
}
