// This file is part of Substrate.

// Copyright (C) 2019-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! This crate provides an implementation of `WasmModule` that is baked by wasmi.

use std::{cell::RefCell, str, sync::Arc};

use log::{debug, error, trace};
use wasmi::{
	memory_units::Pages,
	FuncInstance, ImportsBuilder, MemoryInstance, MemoryRef, Module, ModuleInstance, ModuleRef,
	RuntimeValue::{self, I32, I64},
	TableRef,
};

use sc_allocator::AllocationStats;
use sc_executor_common::{
	error::{Error, MessageWithBacktrace, WasmError},
	runtime_blob::{DataSegmentsSnapshot, RuntimeBlob},
	wasm_runtime::{InvokeMethod, WasmInstance, WasmModule},
};
use sp_runtime_interface::unpack_ptr_and_len;
use sp_wasm_interface::{Function, FunctionContext, Pointer, Result as WResult, WordSize};

struct FunctionExecutor {
	heap: RefCell<sc_allocator::FreeingBumpHeapAllocator>,
	memory: MemoryRef,
	host_functions: Arc<Vec<&'static dyn Function>>,
	allow_missing_func_imports: bool,
	missing_functions: Arc<Vec<String>>,
	panic_message: Option<String>,
}

impl FunctionExecutor {
	fn new(
		m: MemoryRef,
		heap_base: u32,
		host_functions: Arc<Vec<&'static dyn Function>>,
		allow_missing_func_imports: bool,
		missing_functions: Arc<Vec<String>>,
	) -> Result<Self, Error> {
		Ok(FunctionExecutor {
			heap: RefCell::new(sc_allocator::FreeingBumpHeapAllocator::new(heap_base)),
			memory: m,
			host_functions,
			allow_missing_func_imports,
			missing_functions,
			panic_message: None,
		})
	}
}

impl FunctionContext for FunctionExecutor {
	fn read_memory_into(&self, address: Pointer<u8>, dest: &mut [u8]) -> WResult<()> {
		self.memory.get_into(address.into(), dest).map_err(|e| e.to_string())
	}

	fn write_memory(&mut self, address: Pointer<u8>, data: &[u8]) -> WResult<()> {
		self.memory.set(address.into(), data).map_err(|e| e.to_string())
	}

	fn allocate_memory(&mut self, size: WordSize) -> WResult<Pointer<u8>> {
		let heap = &mut self.heap.borrow_mut();
		self.memory
			.with_direct_access_mut(|mem| heap.allocate(mem, size).map_err(|e| e.to_string()))
	}

	fn deallocate_memory(&mut self, ptr: Pointer<u8>) -> WResult<()> {
		let heap = &mut self.heap.borrow_mut();
		self.memory
			.with_direct_access_mut(|mem| heap.deallocate(mem, ptr).map_err(|e| e.to_string()))
	}

	fn register_panic_error_message(&mut self, message: &str) {
		self.panic_message = Some(message.to_owned());
	}
}

/// Will be used on initialization of a module to resolve function and memory imports.
struct Resolver<'a> {
	/// All the hot functions that we export for the WASM blob.
	host_functions: &'a [&'static dyn Function],
	/// Should we allow missing function imports?
	///
	/// If `true`, we return a stub that will return an error when being called.
	allow_missing_func_imports: bool,
	/// All the names of functions for that we did not provide a host function.
	missing_functions: RefCell<Vec<String>>,
	/// Will be used as initial and maximum size of the imported memory.
	heap_pages: usize,
	/// By default, runtimes should import memory and this is `Some(_)` after
	/// resolving. However, to be backwards compatible, we also support memory
	/// exported by the WASM blob (this will be `None` after resolving).
	import_memory: RefCell<Option<MemoryRef>>,
}

impl<'a> Resolver<'a> {
	fn new(
		host_functions: &'a [&'static dyn Function],
		allow_missing_func_imports: bool,
		heap_pages: usize,
	) -> Resolver<'a> {
		Resolver {
			host_functions,
			allow_missing_func_imports,
			missing_functions: RefCell::new(Vec::new()),
			heap_pages,
			import_memory: Default::default(),
		}
	}
}

impl<'a> wasmi::ModuleImportResolver for Resolver<'a> {
	fn resolve_func(
		&self,
		name: &str,
		signature: &wasmi::Signature,
	) -> std::result::Result<wasmi::FuncRef, wasmi::Error> {
		let signature = sp_wasm_interface::Signature::from(signature);
		for (function_index, function) in self.host_functions.iter().enumerate() {
			if name == function.name() {
				if signature == function.signature() {
					return Ok(wasmi::FuncInstance::alloc_host(signature.into(), function_index))
				} else {
					return Err(wasmi::Error::Instantiation(format!(
						"Invalid signature for function `{}` expected `{:?}`, got `{:?}`",
						function.name(),
						signature,
						function.signature(),
					)))
				}
			}
		}

		if self.allow_missing_func_imports {
			trace!(target: "wasm-executor", "Could not find function `{}`, a stub will be provided instead.", name);
			let id = self.missing_functions.borrow().len() + self.host_functions.len();
			self.missing_functions.borrow_mut().push(name.to_string());

			Ok(wasmi::FuncInstance::alloc_host(signature.into(), id))
		} else {
			Err(wasmi::Error::Instantiation(format!("Export {} not found", name)))
		}
	}

	fn resolve_memory(
		&self,
		field_name: &str,
		memory_type: &wasmi::MemoryDescriptor,
	) -> Result<MemoryRef, wasmi::Error> {
		if field_name == "memory" {
			match &mut *self.import_memory.borrow_mut() {
				Some(_) =>
					Err(wasmi::Error::Instantiation("Memory can not be imported twice!".into())),
				memory_ref @ None => {
					if memory_type
						.maximum()
						.map(|m| m.saturating_sub(memory_type.initial()))
						.map(|m| self.heap_pages > m as usize)
						.unwrap_or(false)
					{
						Err(wasmi::Error::Instantiation(format!(
							"Heap pages ({}) is greater than imported memory maximum ({}).",
							self.heap_pages,
							memory_type
								.maximum()
								.map(|m| m.saturating_sub(memory_type.initial()))
								.expect("Maximum is set, checked above; qed"),
						)))
					} else {
						let memory = MemoryInstance::alloc(
							Pages(memory_type.initial() as usize + self.heap_pages),
							Some(Pages(memory_type.initial() as usize + self.heap_pages)),
						)?;
						*memory_ref = Some(memory.clone());
						Ok(memory)
					}
				},
			}
		} else {
			Err(wasmi::Error::Instantiation(format!(
				"Unknown memory reference with name: {}",
				field_name
			)))
		}
	}
}

impl wasmi::Externals for FunctionExecutor {
	fn invoke_index(
		&mut self,
		index: usize,
		args: wasmi::RuntimeArgs,
	) -> Result<Option<wasmi::RuntimeValue>, wasmi::Trap> {
		let mut args = args.as_ref().iter().copied().map(Into::into);

		if let Some(function) = self.host_functions.clone().get(index) {
			function
				.execute(self, &mut args)
				.map_err(|msg| Error::FunctionExecution(function.name().to_string(), msg))
				.map_err(wasmi::Trap::from)
				.map(|v| v.map(Into::into))
		} else if self.allow_missing_func_imports &&
			index >= self.host_functions.len() &&
			index < self.host_functions.len() + self.missing_functions.len()
		{
			Err(Error::from(format!(
				"Function `{}` is only a stub. Calling a stub is not allowed.",
				self.missing_functions[index - self.host_functions.len()],
			))
			.into())
		} else {
			Err(Error::from(format!("Could not find host function with index: {}", index)).into())
		}
	}
}

fn get_mem_instance(module: &ModuleRef) -> Result<MemoryRef, Error> {
	Ok(module
		.export_by_name("memory")
		.ok_or(Error::InvalidMemoryReference)?
		.as_memory()
		.ok_or(Error::InvalidMemoryReference)?
		.clone())
}

/// Find the global named `__heap_base` in the given wasm module instance and
/// tries to get its value.
fn get_heap_base(module: &ModuleRef) -> Result<u32, Error> {
	let heap_base_val = module
		.export_by_name("__heap_base")
		.ok_or(Error::HeapBaseNotFoundOrInvalid)?
		.as_global()
		.ok_or(Error::HeapBaseNotFoundOrInvalid)?
		.get();

	match heap_base_val {
		wasmi::RuntimeValue::I32(v) => Ok(v as u32),
		_ => Err(Error::HeapBaseNotFoundOrInvalid),
	}
}

/// Call a given method in the given wasm-module runtime.
fn call_in_wasm_module(
	module_instance: &ModuleRef,
	memory: &MemoryRef,
	method: InvokeMethod,
	data: &[u8],
	host_functions: Arc<Vec<&'static dyn Function>>,
	allow_missing_func_imports: bool,
	missing_functions: Arc<Vec<String>>,
	allocation_stats: &mut Option<AllocationStats>,
) -> Result<Vec<u8>, Error> {
	// Initialize FunctionExecutor.
	let table: Option<TableRef> = module_instance
		.export_by_name("__indirect_function_table")
		.and_then(|e| e.as_table().cloned());
	let heap_base = get_heap_base(module_instance)?;

	let mut function_executor = FunctionExecutor::new(
		memory.clone(),
		heap_base,
		host_functions,
		allow_missing_func_imports,
		missing_functions,
	)?;

	// Write the call data
	let offset = function_executor.allocate_memory(data.len() as u32)?;
	function_executor.write_memory(offset, data)?;

	fn convert_trap(executor: &mut FunctionExecutor, trap: wasmi::Trap) -> Error {
		if let Some(message) = executor.panic_message.take() {
			Error::AbortedDueToPanic(MessageWithBacktrace { message, backtrace: None })
		} else {
			Error::AbortedDueToTrap(MessageWithBacktrace {
				message: trap.to_string(),
				backtrace: None,
			})
		}
	}

	let result = match method {
		InvokeMethod::Export(method) => module_instance
			.invoke_export(
				method,
				&[I32(u32::from(offset) as i32), I32(data.len() as i32)],
				&mut function_executor,
			)
			.map_err(|error| {
				if let wasmi::Error::Trap(trap) = error {
					convert_trap(&mut function_executor, trap)
				} else {
					error.into()
				}
			}),
		InvokeMethod::Table(func_ref) => {
			let func = table
				.ok_or(Error::NoTable)?
				.get(func_ref)?
				.ok_or(Error::NoTableEntryWithIndex(func_ref))?;
			FuncInstance::invoke(
				&func,
				&[I32(u32::from(offset) as i32), I32(data.len() as i32)],
				&mut function_executor,
			)
			.map_err(|trap| convert_trap(&mut function_executor, trap))
		},
		InvokeMethod::TableWithWrapper { dispatcher_ref, func } => {
			let dispatcher = table
				.ok_or(Error::NoTable)?
				.get(dispatcher_ref)?
				.ok_or(Error::NoTableEntryWithIndex(dispatcher_ref))?;

			FuncInstance::invoke(
				&dispatcher,
				&[I32(func as _), I32(u32::from(offset) as i32), I32(data.len() as i32)],
				&mut function_executor,
			)
			.map_err(|trap| convert_trap(&mut function_executor, trap))
		},
	};

	*allocation_stats = Some(function_executor.heap.borrow().stats());

	match result {
		Ok(Some(I64(r))) => {
			let (ptr, length) = unpack_ptr_and_len(r as u64);
			#[allow(deprecated)]
			memory.get(ptr, length as usize).map_err(|_| Error::Runtime)
		},
		Err(e) => {
			trace!(
				target: "wasm-executor",
				"Failed to execute code with {} pages",
				memory.current_size().0,
			);
			Err(e)
		},
		_ => Err(Error::InvalidReturn),
	}
}

/// Prepare module instance
fn instantiate_module(
	heap_pages: usize,
	module: &Module,
	host_functions: &[&'static dyn Function],
	allow_missing_func_imports: bool,
) -> Result<(ModuleRef, Vec<String>, MemoryRef), Error> {
	let resolver = Resolver::new(host_functions, allow_missing_func_imports, heap_pages);
	// start module instantiation. Don't run 'start' function yet.
	let intermediate_instance =
		ModuleInstance::new(module, &ImportsBuilder::new().with_resolver("env", &resolver))?;

	// Verify that the module has the heap base global variable.
	let _ = get_heap_base(intermediate_instance.not_started_instance())?;

	// Get the memory reference. Runtimes should import memory, but to be backwards
	// compatible we also support exported memory.
	let memory = match resolver.import_memory.into_inner() {
		Some(memory) => memory,
		None => {
			debug!(
				target: "wasm-executor",
				"WASM blob does not imports memory, falling back to exported memory",
			);

			let memory = get_mem_instance(intermediate_instance.not_started_instance())?;
			memory.grow(Pages(heap_pages)).map_err(|_| Error::Runtime)?;

			memory
		},
	};

	if intermediate_instance.has_start() {
		// Runtime is not allowed to have the `start` function.
		Err(Error::RuntimeHasStartFn)
	} else {
		Ok((
			intermediate_instance.assert_no_start(),
			resolver.missing_functions.into_inner(),
			memory,
		))
	}
}

/// A state snapshot of an instance taken just after instantiation.
///
/// It is used for restoring the state of the module after execution.
#[derive(Clone)]
struct GlobalValsSnapshot {
	/// The list of all global mutable variables of the module in their sequential order.
	global_mut_values: Vec<RuntimeValue>,
}

impl GlobalValsSnapshot {
	// Returns `None` if instance is not valid.
	fn take(module_instance: &ModuleRef) -> Self {
		// Collect all values of mutable globals.
		let global_mut_values = module_instance
			.globals()
			.iter()
			.filter(|g| g.is_mutable())
			.map(|g| g.get())
			.collect();
		Self { global_mut_values }
	}

	/// Reset the runtime instance to the initial version by restoring
	/// the preserved memory and globals.
	///
	/// Returns `Err` if applying the snapshot is failed.
	fn apply(&self, instance: &ModuleRef) -> Result<(), WasmError> {
		for (global_ref, global_val) in instance
			.globals()
			.iter()
			.filter(|g| g.is_mutable())
			.zip(self.global_mut_values.iter())
		{
			// the instance should be the same as used for preserving and
			// we iterate the same way it as we do it for preserving values that means that the
			// types should be the same and all the values are mutable. So no error is expected/
			global_ref.set(*global_val).map_err(|_| WasmError::ApplySnapshotFailed)?;
		}
		Ok(())
	}
}

/// A runtime along with initial copy of data segments.
pub struct WasmiRuntime {
	/// A wasm module.
	module: Module,
	/// The host functions registered for this instance.
	host_functions: Arc<Vec<&'static dyn Function>>,
	/// Enable stub generation for functions that are not available in `host_functions`.
	/// These stubs will error when the wasm blob tries to call them.
	allow_missing_func_imports: bool,
	/// Numer of heap pages this runtime uses.
	heap_pages: u64,

	global_vals_snapshot: GlobalValsSnapshot,
	data_segments_snapshot: DataSegmentsSnapshot,
}

impl WasmModule for WasmiRuntime {
	fn new_instance(&self) -> Result<Box<dyn WasmInstance>, Error> {
		// Instantiate this module.
		let (instance, missing_functions, memory) = instantiate_module(
			self.heap_pages as usize,
			&self.module,
			&self.host_functions,
			self.allow_missing_func_imports,
		)
		.map_err(|e| WasmError::Instantiation(e.to_string()))?;

		Ok(Box::new(WasmiInstance {
			instance,
			memory,
			global_vals_snapshot: self.global_vals_snapshot.clone(),
			data_segments_snapshot: self.data_segments_snapshot.clone(),
			host_functions: self.host_functions.clone(),
			allow_missing_func_imports: self.allow_missing_func_imports,
			missing_functions: Arc::new(missing_functions),
		}))
	}
}

/// Create a new `WasmiRuntime` given the code. This function loads the module and
/// stores it in the instance.
pub fn create_runtime(
	blob: RuntimeBlob,
	heap_pages: u64,
	host_functions: Vec<&'static dyn Function>,
	allow_missing_func_imports: bool,
) -> Result<WasmiRuntime, WasmError> {
	let data_segments_snapshot =
		DataSegmentsSnapshot::take(&blob).map_err(|e| WasmError::Other(e.to_string()))?;

	let module =
		Module::from_parity_wasm_module(blob.into_inner()).map_err(|_| WasmError::InvalidModule)?;

	let global_vals_snapshot = {
		let (instance, _, _) = instantiate_module(
			heap_pages as usize,
			&module,
			&host_functions,
			allow_missing_func_imports,
		)
		.map_err(|e| WasmError::Instantiation(e.to_string()))?;
		GlobalValsSnapshot::take(&instance)
	};

	Ok(WasmiRuntime {
		module,
		data_segments_snapshot,
		global_vals_snapshot,
		host_functions: Arc::new(host_functions),
		allow_missing_func_imports,
		heap_pages,
	})
}

/// Wasmi instance wrapper along with the state snapshot.
pub struct WasmiInstance {
	/// A wasm module instance.
	instance: ModuleRef,
	/// The memory instance of used by the wasm module.
	memory: MemoryRef,
	/// The snapshot of global variable values just after instantiation.
	global_vals_snapshot: GlobalValsSnapshot,
	/// The snapshot of data segments.
	data_segments_snapshot: DataSegmentsSnapshot,
	/// The host functions registered for this instance.
	host_functions: Arc<Vec<&'static dyn Function>>,
	/// Enable stub generation for functions that are not available in `host_functions`.
	/// These stubs will error when the wasm blob trie to call them.
	allow_missing_func_imports: bool,
	/// List of missing functions detected during function resolution
	missing_functions: Arc<Vec<String>>,
}

// This is safe because `WasmiInstance` does not leak any references to `self.memory` and
// `self.instance`
unsafe impl Send for WasmiInstance {}

impl WasmiInstance {
	fn call_impl(
		&mut self,
		method: InvokeMethod,
		data: &[u8],
		allocation_stats: &mut Option<AllocationStats>,
	) -> Result<Vec<u8>, Error> {
		// We reuse a single wasm instance for multiple calls and a previous call (if any)
		// altered the state. Therefore, we need to restore the instance to original state.

		// First, zero initialize the linear memory.
		self.memory.erase().map_err(|e| {
			// Snapshot restoration failed. This is pretty unexpected since this can happen
			// if some invariant is broken or if the system is under extreme memory pressure
			// (so erasing fails).
			error!(target: "wasm-executor", "snapshot restoration failed: {}", e);
			WasmError::ErasingFailed(e.to_string())
		})?;

		// Second, reapply data segments into the linear memory.
		self.data_segments_snapshot
			.apply(|offset, contents| self.memory.set(offset, contents))?;

		// Third, restore the global variables to their initial values.
		self.global_vals_snapshot.apply(&self.instance)?;

		call_in_wasm_module(
			&self.instance,
			&self.memory,
			method,
			data,
			self.host_functions.clone(),
			self.allow_missing_func_imports,
			self.missing_functions.clone(),
			allocation_stats,
		)
	}
}

impl WasmInstance for WasmiInstance {
	fn call_with_allocation_stats(
		&mut self,
		method: InvokeMethod,
		data: &[u8],
	) -> (Result<Vec<u8>, Error>, Option<AllocationStats>) {
		let mut allocation_stats = None;
		let result = self.call_impl(method, data, &mut allocation_stats);
		(result, allocation_stats)
	}

	fn get_global_const(&mut self, name: &str) -> Result<Option<sp_wasm_interface::Value>, Error> {
		match self.instance.export_by_name(name) {
			Some(global) => Ok(Some(
				global
					.as_global()
					.ok_or_else(|| format!("`{}` is not a global", name))?
					.get()
					.into(),
			)),
			None => Ok(None),
		}
	}
}
