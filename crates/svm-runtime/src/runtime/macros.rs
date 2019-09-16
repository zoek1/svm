/// Injects a `svm` runtime into current file.
///
/// * `pages_storage_gen` - a function generating a new `svm_storage::traits::PagesStorage`
///
/// * `page_cache_ctor` - a function generating a new `svm_storage::traits::PageCache`
///     wrapping the `PagesStorage` generated by `pages_storage_gen` above.
///
/// * `PC` - the `PageCache` type used. `page_cache_ctor` returns an instance of `PC`
///
/// * `ENV` - the environment type. This type implements trait `svm_contract::ContractEnv`
///
/// * `env_gen` - a function generating an environment. The environment will be of type `ENV` above.
#[macro_export]
macro_rules! include_svm_runtime {
    ($pages_storage_gen: expr, $page_cache_ctor: expr, $PC: path, $ENV: path, $env_gen: expr) => {
        mod runtime {
            use $crate::runtime::ContractExecError;

            /// Iinjects `vmcalls` module into the current file
            svm_runtime::include_svm_vmcalls!($PC);

            use svm_common::{Address, State};

            use svm_contract::{
                env::ContractEnv,
                error::{ContractBuildError, TransactionBuildError},
                traits::ContractStore,
                transaction::Transaction,
                wasm::Contract,
            };

            #[inline(always)]
            pub fn contract_build(bytes: &[u8]) -> Result<Contract, ContractBuildError> {
                <$ENV as ContractEnv>::build_contract(bytes)
            }

            #[inline(always)]
            pub fn contract_deploy_validate(contract: &Contract) -> Result<(), ContractBuildError> {
                // TODO:
                // validate the `wasm`. should use the `deterministic` feature of `wasmparser`.
                // (avoiding floats etc.)

                Ok(())
            }

            #[inline(always)]
            pub fn contract_compute_address(contract: &Contract) -> Address {
                <$ENV as ContractEnv>::compute_address(contract)
            }

            #[inline(always)]
            pub fn contract_store(contract: &Contract, addr: &Address) {
                let mut env = $env_gen();
                env.store_contract(contract, addr);
            }

            #[inline(always)]
            pub fn transaction_build(bytes: &[u8]) -> Result<Transaction, TransactionBuildError> {
                <$ENV as ContractEnv>::build_transaction(bytes)
            }

            pub fn contract_exec(
                tx: &Transaction,
                import_object: &wasmer_runtime::ImportObject,
            ) -> Result<State, ContractExecError> {
                let mut env = $env_gen();

                dbg!("11111111111111111111111");
                let contract = contract_load(tx, &mut env)?;

                dbg!("22222222222222222222222");
                let module = contract_compile(&contract, &tx.contract)?;

                dbg!("33333333333333333333333");
                let mut instance =
                    module_instantiate(&contract, &tx.contract, &module, import_object)?;

                dbg!("44444444444444444444444");
                let args = prepare_args_and_memory(tx, &mut instance);

                dbg!("555555555555555555555555");
                let func = get_exported_func(&instance, &tx.func_name)?;

                let res = match func.call(&args) {
                    Err(e) => Err(ContractExecError::ExecFailed),
                    Ok(_) => {
                        let storage = get_instance_svm_storage_mut(&mut instance);
                        let state = storage.commit();

                        Ok(state)
                    }
                };

                dbg!(&res);

                res
            }

            pub fn import_object_create(
                addr: Address,
                state: State,
                node_data: *const std::ffi::c_void,
                opts: $crate::opts::Opts,
            ) -> wasmer_runtime::ImportObject {
                use wasmer_runtime::{func, ImportObject};

                let wrapped_pages_storage_gen =
                    move || $pages_storage_gen(addr.clone(), state.clone(), opts.max_pages);

                let state_gen = svm_runtime::lazy_create_svm_state_gen!(
                    node_data,
                    wrapped_pages_storage_gen,
                    $page_cache_ctor,
                    $PC,
                    opts
                );

                let mut import_object = ImportObject::new_with_data(state_gen);

                let mut ns = wasmer_runtime_core::import::Namespace::new();

                // storage
                ns.insert("mem_to_reg_copy", func!(vmcalls::mem_to_reg_copy));
                ns.insert("reg_to_mem_copy", func!(vmcalls::reg_to_mem_copy));
                ns.insert("storage_read_to_reg", func!(vmcalls::storage_read_to_reg));
                ns.insert("storage_read_to_mem", func!(vmcalls::storage_read_to_mem));
                ns.insert(
                    "storage_write_from_mem",
                    func!(vmcalls::storage_write_from_mem),
                );
                ns.insert(
                    "storage_write_from_reg",
                    func!(vmcalls::storage_write_from_reg),
                );

                // register
                ns.insert("reg_read_le_i64", func!(vmcalls::reg_read_le_i64));
                ns.insert("reg_write_le_i64", func!(vmcalls::reg_write_le_i64));

                import_object.register("svm", ns);

                import_object
            }

            fn contract_load(
                tx: &Transaction,
                env: &mut $ENV,
            ) -> Result<Contract, ContractExecError> {
                let store = env.get_store();

                match store.load(&tx.contract) {
                    None => Err(ContractExecError::NotFound(tx.contract.clone())),
                    Some(contract) => Ok(contract),
                }
            }

            fn contract_compile(
                contract: &Contract,
                addr: &Address,
            ) -> Result<wasmer_runtime::Module, ContractExecError> {
                let compile = wasmer_runtime::compile(&contract.wasm);

                match compile {
                    Err(e) => Err(ContractExecError::CompilationFailed(addr.clone())),
                    Ok(module) => Ok(module),
                }
            }

            fn module_instantiate(
                contract: &Contract,
                addr: &Address,
                module: &wasmer_runtime::Module,
                import_object: &wasmer_runtime::ImportObject,
            ) -> Result<wasmer_runtime::Instance, ContractExecError> {
                let instantiate = module.instantiate(&import_object);

                match instantiate {
                    Err(e) => {
                        dbg!(e);
                        Err(ContractExecError::InstantiationFailed(addr.clone()))
                    }
                    Ok(instance) => Ok(instance),
                }
            }

            fn get_exported_func<'a>(
                instance: &'a wasmer_runtime::Instance,
                func_name: &str,
            ) -> Result<wasmer_runtime::DynFunc<'a>, ContractExecError> {
                let func = instance.dyn_func(func_name);

                match func {
                    Err(_) => Err(ContractExecError::FuncNotFound(func_name.to_string())),
                    Ok(func) => Ok(func),
                }
            }

            fn prepare_args_and_memory(
                tx: &Transaction,
                instance: &mut wasmer_runtime::Instance,
            ) -> Vec<wasmer_runtime::Value> {
                use svm_contract::wasm::{WasmArgValue, WasmIntType};
                use wasmer_runtime::Value;

                let memory = instance.context_mut().memory(0);
                let mut mem_offset = 0;

                let mut wasmer_args = Vec::with_capacity(tx.func_args.len());

                for arg in tx.func_args.iter() {
                    let wasmer_arg = match arg {
                        WasmArgValue::I32(v) => Value::I32(*v as i32),
                        WasmArgValue::I64(v) => Value::I64(*v as i64),
                        WasmArgValue::Fixed(ty, buf) => {
                            let buf_mem_start = mem_offset;

                            let view = memory.view();

                            for byte in buf.into_iter() {
                                view[mem_offset].set(*byte);
                                mem_offset += 1;
                            }

                            match ty {
                                WasmIntType::I32 => Value::I32(buf_mem_start as i32),
                                WasmIntType::I64 => Value::I64(buf_mem_start as i64),
                            }
                        }
                        WasmArgValue::Slice(..) => unimplemented!(),
                    };

                    wasmer_args.push(wasmer_arg);
                }

                wasmer_args
            }

            #[inline(always)]
            fn get_instance_svm_storage_mut(
                instance: &mut wasmer_runtime::Instance,
            ) -> &mut svm_storage::PageSliceCache<$PC> {
                let wasmer_ctx: &mut wasmer_runtime::Ctx = instance.context_mut();

                $crate::wasmer_data_storage!(wasmer_ctx.data, $PC)
            }
        }
    };
}
