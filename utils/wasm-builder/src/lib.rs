// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Wasm builder is a utility for building a project as a Wasm binary
//!
//! The Wasm builder is a tool that integrates the process of building the WASM binary of your
//! project into the main `cargo` build process.
//!
//! ## Project setup
//!
//! A project that should be compiled as a Wasm binary needs to:
//!
//! 1. Add a `build.rs` file.
//! 2. Add `wasm-builder` as dependency into `build-dependencies`.
//!
//! The `build.rs` file needs to contain the following code:
//!
//! ```no_run
//! use substrate_wasm_builder::WasmBuilder;
//!
//! fn main() {
//!     WasmBuilder::new()
//!         // Tell the builder to build the project (crate) this `build.rs` is part of.
//!         .with_current_project()
//!         // Make sure to export the `heap_base` global, this is required by Substrate
//!         .export_heap_base()
//!         // Build the Wasm file so that it imports the memory (need to be provided by at instantiation)
//!         .import_memory()
//!         // Build it.
//!         .build()
//! }
//! ```
//!
//! As the final step, you need to add the following to your project:
//!
//! ```ignore
//! include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));
//! ```
//!
//! This will include the generated Wasm binary as two constants `WASM_BINARY` and
//! `WASM_BINARY_BLOATY`. The former is a compact Wasm binary and the latter is the Wasm binary as
//! being generated by the compiler. Both variables have `Option<&'static [u8]>` as type.
//!
//! ### Feature
//!
//! Wasm builder supports to enable cargo features while building the Wasm binary. By default it
//! will enable all features in the wasm build that are enabled for the native build except the
//! `default` and `std` features. Besides that, wasm builder supports the special `runtime-wasm`
//! feature. This `runtime-wasm` feature will be enabled by the wasm builder when it compiles the
//! Wasm binary. If this feature is not present, it will not be enabled.
//!
//! ## Environment variables
//!
//! By using environment variables, you can configure which Wasm binaries are built and how:
//!
//! - `SKIP_WASM_BUILD` - Skips building any Wasm binary. This is useful when only native should be
//!   recompiled. If this is the first run and there doesn't exist a Wasm binary, this will set both
//!   variables to `None`.
//! - `WASM_BUILD_TYPE` - Sets the build type for building Wasm binaries. Supported values are
//!   `release` or `debug`. By default the build type is equal to the build type used by the main
//!   build.
//! - `FORCE_WASM_BUILD` - Can be set to force a Wasm build. On subsequent calls the value of the
//!   variable needs to change. As wasm-builder instructs `cargo` to watch for file changes this
//!   environment variable should only be required in certain circumstances.
//! - `WASM_BUILD_RUSTFLAGS` - Extend `RUSTFLAGS` given to `cargo build` while building the wasm
//!   binary.
//! - `WASM_BUILD_NO_COLOR` - Disable color output of the wasm build.
//! - `WASM_TARGET_DIRECTORY` - Will copy any build Wasm binary to the given directory. The path
//!   needs to be absolute.
//! - `WASM_BUILD_WORKSPACE_HINT` - Hint the workspace that is being built. This is normally not
//!   required as we walk up from the target directory until we find a `Cargo.toml`. If the target
//!   directory is changed for the build, this environment variable can be used to point to the
//!   actual workspace.
//! - `CARGO_NET_OFFLINE` - If `true`, `--offline` will be passed to all processes launched to
//!   prevent network access. Useful in offline environments.
//!
//! Each project can be skipped individually by using the environment variable
//! `SKIP_PROJECT_NAME_WASM_BUILD`. Where `PROJECT_NAME` needs to be replaced by the name of the
//! cargo project, e.g. `kitchensink-runtime` will be `NODE_RUNTIME`.
//!
//! ## Prerequisites:
//!
//! Wasm builder requires the following prerequisites for building the Wasm binary:
//!
//! - rust nightly + `wasm32-unknown-unknown` toolchain
//!
//! If a specific rust nightly is installed with `rustup`, it is important that the wasm target is
//! installed as well. For example if installing the rust nightly from 20.02.2020 using `rustup
//! install nightly-2020-02-20`, the wasm target needs to be installed as well `rustup target add
//! wasm32-unknown-unknown --toolchain nightly-2020-02-20`.

use std::{
	env, fs,
	path::{Path, PathBuf},
	process::Command,
};

mod builder;
mod prerequisites;
mod wasm_project;

pub use builder::{WasmBuilder, WasmBuilderSelectProject};

/// Environment variable that tells us to skip building the wasm binary.
const SKIP_BUILD_ENV: &str = "SKIP_WASM_BUILD";

/// Environment variable that tells us whether we should avoid network requests
const OFFLINE: &str = "CARGO_NET_OFFLINE";

/// Environment variable to force a certain build type when building the wasm binary.
/// Expects "debug", "release" or "production" as value.
///
/// When unset the WASM binary uses the same build type as the main cargo build with
/// the exception of a debug build: In this case the wasm build defaults to `release` in
/// order to avoid a slowdown when not explicitly requested.
const WASM_BUILD_TYPE_ENV: &str = "WASM_BUILD_TYPE";

/// Environment variable to extend the `RUSTFLAGS` variable given to the wasm build.
const WASM_BUILD_RUSTFLAGS_ENV: &str = "WASM_BUILD_RUSTFLAGS";

/// Environment variable to set the target directory to copy the final wasm binary.
///
/// The directory needs to be an absolute path.
const WASM_TARGET_DIRECTORY: &str = "WASM_TARGET_DIRECTORY";

/// Environment variable to disable color output of the wasm build.
const WASM_BUILD_NO_COLOR: &str = "WASM_BUILD_NO_COLOR";

/// Environment variable that makes sure the WASM build is triggered.
const FORCE_WASM_BUILD_ENV: &str = "FORCE_WASM_BUILD";

/// Environment variable that hints the workspace we are building.
const WASM_BUILD_WORKSPACE_HINT: &str = "WASM_BUILD_WORKSPACE_HINT";

/// Write to the given `file` if the `content` is different.
fn write_file_if_changed(file: impl AsRef<Path>, content: impl AsRef<str>) {
	if fs::read_to_string(file.as_ref()).ok().as_deref() != Some(content.as_ref()) {
		fs::write(file.as_ref(), content.as_ref())
			.unwrap_or_else(|_| panic!("Writing `{}` can not fail!", file.as_ref().display()));
	}
}

/// Copy `src` to `dst` if the `dst` does not exist or is different.
fn copy_file_if_changed(src: PathBuf, dst: PathBuf) {
	let src_file = fs::read_to_string(&src).ok();
	let dst_file = fs::read_to_string(&dst).ok();

	if src_file != dst_file {
		fs::copy(&src, &dst).unwrap_or_else(|_| {
			panic!("Copying `{}` to `{}` can not fail; qed", src.display(), dst.display())
		});
	}
}

/// Wraps a specific command which represents a cargo invocation.
#[derive(Debug)]
struct CargoCommand {
	program: String,
}

impl CargoCommand {
	fn command(&self) -> Command {
		Command::new(&self.program)
	}
}

impl Default for CargoCommand {
	fn default() -> Self {
		Self {
			program: env::var("CARGO").expect("`CARGO` env variable is always set by cargo").into(),
		}
	}
}

/// Wraps a [`CargoCommand`] and the version of `rustc` the cargo command uses.
struct CargoCommandVersioned {
	command: CargoCommand,
	version: String,
}

impl CargoCommandVersioned {
	fn new(command: CargoCommand, version: String) -> Self {
		Self { command, version }
	}

	/// Returns the `rustc` version.
	fn rustc_version(&self) -> &str {
		&self.version
	}
}

impl std::ops::Deref for CargoCommandVersioned {
	type Target = CargoCommand;

	fn deref(&self) -> &CargoCommand {
		&self.command
	}
}

/// Returns `true` when color output is enabled.
fn color_output_enabled() -> bool {
	env::var(crate::WASM_BUILD_NO_COLOR).is_err()
}
