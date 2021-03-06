use wasmer_runtime_core::error::CompileResult;
use wasmer_runtime_core::Module;

/// The `svm_compiler` macro returns a `wasmer singlepass compiler` with middlewares required by the `svm`.
/// Since we can't say explicitly all the wildcards (`_`) we can't define a function
/// returning a `StreamingCompiler<SinglePassMCG, _, _, _, _>` so we use a rust macro instead.
macro_rules! svm_compiler {
    () => {{
        use crate::middleware::ValidationMiddleware;
        use wasmer_runtime_core::codegen::{MiddlewareChain, StreamingCompiler};
        use wasmer_singlepass_backend::ModuleCodeGenerator as SinglePassMCG;

        let compiler: StreamingCompiler<SinglePassMCG, _, _, _, _> =
            StreamingCompiler::new(move || {
                let mut chain = MiddlewareChain::new();
                chain.push(ValidationMiddleware::new());
                chain
            });

        compiler
    }};
}

/// This function is responsible on compiling a wasm program using the `wasmer singlepass` compiler along
/// with the the middlewares required by `svm`.
#[must_use]
pub fn compile_program(wasm: &[u8]) -> CompileResult<Module> {
    let compiler = svm_compiler!();

    wasmer_runtime_core::compile_with(wasm, &compiler)
}
