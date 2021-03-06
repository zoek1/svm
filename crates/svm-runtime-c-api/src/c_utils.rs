use wasmer_runtime_c_api::{
    export::{wasmer_import_export_kind, wasmer_import_export_value},
    import::{wasmer_import_func_t, wasmer_import_t},
    wasmer_byte_array,
};

#[doc(hidden)]
pub fn cast_str_to_wasmer_byte_array(s: &str) -> wasmer_byte_array {
    let bytes: &[u8] = s.as_bytes();
    let bytes_ptr: *const u8 = bytes.as_ptr();
    let bytes_len: u32 = bytes.len() as u32;

    std::mem::forget(bytes);

    wasmer_byte_array {
        bytes: bytes_ptr,
        bytes_len,
    }
}

#[doc(hidden)]
pub unsafe fn cast_wasmer_byte_array_to_string(wasmer_bytes: &wasmer_byte_array) -> String {
    let slice: &[u8] =
        std::slice::from_raw_parts(wasmer_bytes.bytes, wasmer_bytes.bytes_len as usize);

    if let Ok(s) = std::str::from_utf8(slice) {
        s.to_string()
    } else {
        panic!("error converting `wasmer_byte_array` to string")
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! cast_vmcall_to_import_func_t {
    ($func: path, $params: expr, $returns: expr) => {{
        use std::sync::Arc;
        use wasmer_runtime_c_api::import::wasmer_import_func_t;
        use wasmer_runtime_core::{
            export::{Context, Export, FuncPointer},
            types::FuncSig,
        };

        let export = Box::new(Export::Function {
            func: FuncPointer::new($func as _),
            ctx: Context::Internal,
            signature: Arc::new(FuncSig::new($params, $returns)),
        });

        Box::into_raw(export) as *const wasmer_import_func_t
    }};
}

/// a utility function to be used in `c-api` tests
pub fn build_wasmer_import_t(
    mode_name: &str,
    import_name: &str,
    func: *const wasmer_import_func_t,
) -> wasmer_import_t {
    wasmer_import_t {
        module_name: cast_str_to_wasmer_byte_array(mode_name),
        import_name: cast_str_to_wasmer_byte_array(import_name),
        tag: wasmer_import_export_kind::WASM_FUNCTION,
        value: wasmer_import_export_value { func },
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! alloc_raw_ptr_stack {
    ($ptr_struct_type: path, $) => {{
        use std::alloc::Layout;

        let ptr_size: usize = std::mem::size_of::<*mut *mut $ptr_type>();
        let layout = Layout::from_size_align(ptr_size, std::mem::align_of::<u8>()).unwrap();
        let raw_ptr: *mut *mut $ptr_type = unsafe { std::alloc::alloc(layout) as _ };
        raw_ptr
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! alloc_raw_ptr_heap {
    ($ptr_type: path) => {{
        use std::alloc::Layout;

        let ptr_size: usize = std::mem::size_of::<*mut *mut $ptr_type>();
        let layout = Layout::from_size_align(ptr_size, std::mem::align_of::<u8>()).unwrap();
        let raw_ptr: *mut *mut $ptr_type = unsafe { std::alloc::alloc(layout) as _ };

        raw_ptr
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! alloc_raw_contract {
    () => {{
        alloc_raw_ptr_heap!($crate::c_types::svm_contract_t)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! alloc_raw_transaction {
    () => {{
        alloc_raw_ptr_heap!($crate::c_types::svm_transaction_t)
    }};
}
#[doc(hidden)]
#[macro_export]
macro_rules! alloc_raw_receipt {
    () => {{
        alloc_raw_ptr_heap!($crate::c_types::svm_receipt_t)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! alloc_raw_module {
    () => {{
        alloc_raw_ptr_heap!(wasmer_runtime_c_api::module::wasmer_module_t)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! alloc_raw_instance {
    () => {{
        alloc_raw_ptr_heap!(wasmer_runtime_c_api::instance::wasmer_instance_t)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! alloc_raw_import_object {
    () => {{
        alloc_raw_ptr_heap!(wasmer_runtime_c_api::import::wasmer_import_object_t)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! deref_import_obj {
    ($raw_import_object: expr) => {{
        use wasmer_runtime::ImportObject;
        use wasmer_runtime_c_api::import::wasmer_import_object_t;

        let import_obj: &mut ImportObject = &mut *(*$raw_import_object as *mut _);
        import_obj as *const ImportObject as *const wasmer_import_object_t
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! deref_instance {
    ($raw_instance: expr) => {{
        use wasmer_runtime::Instance;

        &mut *(*$raw_instance as *mut Instance)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! wasmer_compile_module {
    ($wasm:expr) => {{
        use wasmer_runtime_c_api::module::wasmer_module_t;

        let mut wasm = wabt::wat2wasm(&$wasm).unwrap();

        let wasm_bytes = wasm.as_mut_ptr();
        let wasm_bytes_len = wasm.len() as u32;
        let raw_module = alloc_raw_module();
        let compile_res = svm_compile(raw_module, wasm_bytes, wasm_bytes_len);

        // TODO: assert `compile_res` is OK`
        // assert_eq!(wasmer_result_t::WASMER_OK, compile_res);

        let module: *const wasmer_module_t = *raw_module as *const _;
        module
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! wasmer_compile_module_file {
    ($file:expr) => {{
        let wasm = include_str!($file);
        wasmer_compile_module!(wasm)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cast_string_to_wasmer_by_array() {
        let module_bytes = cast_str_to_wasmer_byte_array("env");
        let module_str = unsafe { cast_wasmer_byte_array_to_string(&module_bytes) };

        assert_eq!("env", module_str.as_str());
    }
}
