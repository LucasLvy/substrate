// This file is part of Substrate.

// Copyright (C) 2018-2022 Parity Technologies (UK) Ltd.
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

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, ItemImpl, ItemTrait, Result};

use crate::{
	common::CHANGED_IN_ATTRIBUTE,
	utils::{
		extract_impl_trait, filter_cfg_attributes, generate_runtime_mod_name_for_trait,
		get_doc_literals, RequireQualifiedTraitPath,
	},
};

/// Collect extra where bounds on the parameter type.
///
/// `decl_runtime_apis` macro extends the generics of each trait by adding
/// the generic `Block: BlockT`.
///
/// If the generic `Block` is present on the parameter type,
/// then returns the type without any lifetimes or mutability.
/// Otherwise, returns `None`.
fn collect_where_bounds(ty: &syn::Type) -> Option<syn::Type> {
	let ty_string = format!("{}", quote!(#ty));
	if !ty_string.contains("Block") {
		return None
	}

	// Remove the lifetime and mutability of the type T to
	// place bounds around it.
	let ty_elem = match &ty {
		syn::Type::Reference(reference) => &reference.elem,
		syn::Type::Ptr(ptr) => &ptr.elem,
		syn::Type::Slice(slice) => &slice.elem,
		syn::Type::Array(arr) => &arr.elem,
		_ => ty,
	};

	return Some(ty_elem.clone())
}

/// Extract the documentation from the provided attributes.
///
/// It takes into account the `no-metadata-docs` feature.
fn collect_docs(attrs: &[syn::Attribute], crate_: &TokenStream) -> TokenStream {
	if cfg!(feature = "no-metadata-docs") {
		quote!(#crate_::vec![])
	} else {
		let docs = get_doc_literals(&attrs);
		quote!(#crate_::vec![ #( #docs, )* ])
	}
}

/// Generate the runtime metadata of the provided trait.
///
/// The metadata is exposed as a generic function on the hidden module
/// of the trait generated by the `decl_runtime_apis`.
pub fn generate_decl_runtime_metadata(decl: &ItemTrait, crate_: &TokenStream) -> TokenStream {
	let mut methods = Vec::new();

	// Ensure that any function parameter that relies on the `BlockT` bounds
	// also has `TypeInfo + 'static` bounds (required by `scale_info::meta_type`).
	//
	// For example, if a runtime API defines a method that has an input:
	// `fn func(input: <Block as BlockT>::Header)`
	// then the runtime metadata will imply `<Block as BlockT>::Header: TypeInfo + 'static`.
	//
	// This restricts the bounds at the metadata level, without needing to modify the `BlockT`
	// itself, since the concrete implementations are already satisfying `TypeInfo`.
	let mut where_clause = Vec::new();
	for item in &decl.items {
		// Collect metadata for methods only.
		let syn::TraitItem::Method(method) = item else {
			continue
		};

		// Collect metadata only for the latest methods.
		let is_changed_in =
			method.attrs.iter().any(|attr| attr.path.is_ident(CHANGED_IN_ATTRIBUTE));
		if is_changed_in {
			continue
		}

		let mut inputs = Vec::new();
		let signature = &method.sig;
		for input in &signature.inputs {
			// Exclude `self` from metadata collection.
			let syn::FnArg::Typed(typed) = input else {
				continue
			};

			let pat = &typed.pat;
			let name = format!("{}", quote!(#pat));
			let ty = &typed.ty;
			collect_where_bounds(ty).map(|ty_elem| where_clause.push(ty_elem));

			inputs.push(quote!(
				#crate_::metadata_ir::RuntimeApiMethodParamMetadataIR {
					name: #name,
					ty: #crate_::scale_info::meta_type::<#ty>(),
				}
			));
		}

		let output = match &signature.output {
			syn::ReturnType::Default => quote!(#crate_::scale_info::meta_type::<()>()),
			syn::ReturnType::Type(_, ty) => {
				collect_where_bounds(ty).map(|ty_elem| where_clause.push(ty_elem));
				quote!(#crate_::scale_info::meta_type::<#ty>())
			},
		};

		// String method name including quotes for constructing `v15::RuntimeApiMethodMetadata`.
		let method_name = format!("{}", signature.ident);
		let docs = collect_docs(&method.attrs, &crate_);

		// Include the method metadata only if its `cfg` features are enabled.
		let attrs = filter_cfg_attributes(&method.attrs);
		methods.push(quote!(
			#( #attrs )*
			#crate_::metadata_ir::RuntimeApiMethodMetadataIR {
				name: #method_name,
				inputs: #crate_::vec![ #( #inputs, )* ],
				output: #output,
				docs: #docs,
			}
		));
	}

	let trait_name_ident = &decl.ident;
	let trait_name = format!("{}", trait_name_ident);
	let docs = collect_docs(&decl.attrs, &crate_);
	let attrs = filter_cfg_attributes(&decl.attrs);
	// The trait generics where already extended with `Block: BlockT`.
	let mut generics = decl.generics.clone();
	for generic_param in generics.params.iter_mut() {
		let syn::GenericParam::Type(ty) = generic_param else {
			continue
		};

		// `scale_info::meta_type` requires `T: ?Sized + TypeInfo + 'static` bounds.
		ty.bounds.push(parse_quote!(#crate_::scale_info::TypeInfo));
		ty.bounds.push(parse_quote!('static));
	}

	let where_clause: Vec<_> = where_clause
		.iter()
		.map(|ty| quote!(#ty: #crate_::scale_info::TypeInfo + 'static))
		.collect();

	quote!(
		#( #attrs )*
		#[inline(always)]
		pub fn runtime_metadata #generics () -> #crate_::metadata_ir::RuntimeApiMetadataIR
		where #( #where_clause, )*
		{
			#crate_::metadata_ir::RuntimeApiMetadataIR {
				name: #trait_name,
				methods: #crate_::vec![ #( #methods, )* ],
				docs: #docs,
			}
		}
	)
}

/// Implement the `runtime_metadata` function on the runtime that
/// generates the metadata for the given traits.
///
/// The metadata of each trait is extracted from the generic function
/// exposed by `generate_decl_runtime_metadata`.
pub fn generate_impl_runtime_metadata(
	impls: &[ItemImpl],
	crate_: &TokenStream,
) -> Result<TokenStream> {
	if impls.is_empty() {
		return Ok(quote!())
	}

	// Get the name of the runtime for which the traits are implemented.
	let runtime_name = &impls
		.get(0)
		.expect("Traits should contain at least one implementation; qed")
		.self_ty;

	let mut metadata = Vec::new();

	for impl_ in impls {
		let mut trait_ = extract_impl_trait(&impl_, RequireQualifiedTraitPath::Yes)?.clone();

		// Implementation traits are always references with a path `impl client::Core<generics> ...`
		// The trait name is the last segment of this path.
		let trait_name_ident = &trait_
			.segments
			.last()
			.as_ref()
			.expect("Trait path should always contain at least one item; qed")
			.ident;

		// Extract the generics from the trait to pass to the `runtime_metadata`
		// function on the hidden module.
		let generics = trait_
			.segments
			.iter()
			.find_map(|segment| {
				if let syn::PathArguments::AngleBracketed(generics) = &segment.arguments {
					Some(generics.clone())
				} else {
					None
				}
			})
			.expect("Trait path should always contain at least one generic parameter; qed");

		let mod_name = generate_runtime_mod_name_for_trait(&trait_name_ident);
		// Get absolute path to the `runtime_decl_for_` module by replacing the last segment.
		if let Some(segment) = trait_.segments.last_mut() {
			*segment = parse_quote!(#mod_name);
		}

		let attrs = filter_cfg_attributes(&impl_.attrs);
		metadata.push(quote!(
			#( #attrs )*
			#trait_::runtime_metadata::#generics()
		));
	}

	Ok(quote!(
		trait InternalImplRuntimeApis {
			#[inline(always)]
			fn runtime_metadata(&self) -> #crate_::vec::Vec<#crate_::metadata_ir::RuntimeApiMetadataIR> {
				#crate_::vec![ #( #metadata, )* ]
			}
		}
		impl InternalImplRuntimeApis for #runtime_name {}
	))
}
