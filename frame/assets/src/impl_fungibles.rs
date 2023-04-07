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

//! Implementations for fungibles trait.

use super::*;

impl<T: Config<I>, I: 'static> fungibles::Inspect<<T as SystemConfig>::AccountId> for Pallet<T, I> {
	type AssetId = T::AssetId;
	type Balance = T::Balance;

	fn total_issuance(_asset: Self::AssetId) -> Self::Balance {
		Self::Balance::default()
	}

	fn minimum_balance(asset: Self::AssetId) -> Self::Balance {
		Asset::<T, I>::get(asset).map(|x| x.min_balance).unwrap_or_else(Zero::zero)
	}

	fn balance(asset: Self::AssetId, who: &<T as SystemConfig>::AccountId) -> Self::Balance {
		Pallet::<T, I>::balance(asset, who)
	}

	fn reducible_balance(
		asset: Self::AssetId,
		who: &<T as SystemConfig>::AccountId,
		keep_alive: bool,
	) -> Self::Balance {
		Pallet::<T, I>::reducible_balance(asset, who, keep_alive).unwrap_or(Zero::zero())
	}

	fn can_deposit(
		asset: Self::AssetId,
		who: &<T as SystemConfig>::AccountId,
		amount: Self::Balance,
		mint: bool,
	) -> DepositConsequence {
		Pallet::<T, I>::can_increase(asset, who, amount, mint)
	}

	fn can_withdraw(
		asset: Self::AssetId,
		who: &<T as SystemConfig>::AccountId,
		amount: Self::Balance,
	) -> WithdrawConsequence<Self::Balance> {
		Pallet::<T, I>::can_decrease(asset, who, amount, false)
	}

	fn asset_exists(asset: Self::AssetId) -> bool {
		Asset::<T, I>::contains_key(asset)
	}
}

impl<T: Config<I>, I: 'static> fungibles::InspectMetadata<<T as SystemConfig>::AccountId>
	for Pallet<T, I>
{
	/// Return the name of an asset.
	fn name(asset: &Self::AssetId) -> Vec<u8> {
		Metadata::<T, I>::get(asset).name.to_vec()
	}

	/// Return the symbol of an asset.
	fn symbol(asset: &Self::AssetId) -> Vec<u8> {
		Metadata::<T, I>::get(asset).symbol.to_vec()
	}

	/// Return the decimals of an asset.
	fn decimals(asset: &Self::AssetId) -> u8 {
		Metadata::<T, I>::get(asset).decimals
	}
}

impl<T: Config<I>, I: 'static> fungibles::Mutate<<T as SystemConfig>::AccountId> for Pallet<T, I> {
	fn mint_into(
		asset: Self::AssetId,
		who: &<T as SystemConfig>::AccountId,
		amount: Self::Balance,
	) -> DispatchResult {
		Self::do_mint(asset, who, amount, None)
	}

	fn burn_from(
		asset: Self::AssetId,
		who: &<T as SystemConfig>::AccountId,
		amount: Self::Balance,
	) -> Result<Self::Balance, DispatchError> {
		let f = DebitFlags { keep_alive: false, best_effort: false };
		Self::do_burn(asset, who, amount, None, f)
	}

	fn slash(
		asset: Self::AssetId,
		who: &<T as SystemConfig>::AccountId,
		amount: Self::Balance,
	) -> Result<Self::Balance, DispatchError> {
		let f = DebitFlags { keep_alive: false, best_effort: true };
		Self::do_burn(asset, who, amount, None, f)
	}
}

impl<T: Config<I>, I: 'static> fungibles::Transfer<T::AccountId> for Pallet<T, I> {
	fn transfer(
		asset: Self::AssetId,
		source: &T::AccountId,
		dest: &T::AccountId,
		amount: T::Balance,
		keep_alive: bool,
	) -> Result<T::Balance, DispatchError> {
		let f = TransferFlags { keep_alive, best_effort: false, burn_dust: false };
		Self::do_transfer(asset, source, dest, amount, None, f)
	}
}

impl<T: Config<I>, I: 'static> fungibles::Unbalanced<T::AccountId> for Pallet<T, I> {
	fn set_balance(_: Self::AssetId, _: &T::AccountId, _: Self::Balance) -> DispatchResult {
		unreachable!("set_balance is not used if other functions are impl'd");
	}
	fn set_total_issuance(_id: T::AssetId, _amount: Self::Balance) {}
	fn decrease_balance(
		_asset: T::AssetId,
		_who: &T::AccountId,
		_amount: Self::Balance,
	) -> Result<Self::Balance, DispatchError> {
		Err(DispatchError::NotImplementedForStarknet)
	}
	fn decrease_balance_at_most(
		_asset: T::AssetId,
		_who: &T::AccountId,
		_amount: Self::Balance,
	) -> Self::Balance {
		Self::Balance::default()
	}
	fn increase_balance(
		_asset: T::AssetId,
		_who: &T::AccountId,
		_amount: Self::Balance,
	) -> Result<Self::Balance, DispatchError> {
		Err(DispatchError::NotImplementedForStarknet)
	}
	fn increase_balance_at_most(
		_asset: T::AssetId,
		_who: &T::AccountId,
		_amount: Self::Balance,
	) -> Self::Balance {
		Self::Balance::default()
	}
}

impl<T: Config<I>, I: 'static> fungibles::Create<T::AccountId> for Pallet<T, I> {
	fn create(
		id: T::AssetId,
		admin: T::AccountId,
		is_sufficient: bool,
		min_balance: Self::Balance,
	) -> DispatchResult {
		Self::do_force_create(id, admin, is_sufficient, min_balance)
	}
}

impl<T: Config<I>, I: 'static> fungibles::Destroy<T::AccountId> for Pallet<T, I> {
	fn start_destroy(id: T::AssetId, maybe_check_owner: Option<T::AccountId>) -> DispatchResult {
		Self::do_start_destroy(id, maybe_check_owner)
	}

	fn destroy_accounts(id: T::AssetId, max_items: u32) -> Result<u32, DispatchError> {
		Self::do_destroy_accounts(id, max_items)
	}

	fn destroy_approvals(id: T::AssetId, max_items: u32) -> Result<u32, DispatchError> {
		Self::do_destroy_approvals(id, max_items)
	}

	fn finish_destroy(id: T::AssetId) -> DispatchResult {
		Self::do_finish_destroy(id)
	}
}

impl<T: Config<I>, I: 'static> fungibles::metadata::Inspect<<T as SystemConfig>::AccountId>
	for Pallet<T, I>
{
	fn name(asset: T::AssetId) -> Vec<u8> {
		Metadata::<T, I>::get(asset).name.to_vec()
	}

	fn symbol(asset: T::AssetId) -> Vec<u8> {
		Metadata::<T, I>::get(asset).symbol.to_vec()
	}

	fn decimals(asset: T::AssetId) -> u8 {
		Metadata::<T, I>::get(asset).decimals
	}
}

impl<T: Config<I>, I: 'static> fungibles::metadata::Mutate<<T as SystemConfig>::AccountId>
	for Pallet<T, I>
{
	fn set(
		asset: T::AssetId,
		from: &<T as SystemConfig>::AccountId,
		name: Vec<u8>,
		symbol: Vec<u8>,
		decimals: u8,
	) -> DispatchResult {
		Self::do_set_metadata(asset, from, name, symbol, decimals)
	}
}

impl<T: Config<I>, I: 'static> fungibles::approvals::Inspect<<T as SystemConfig>::AccountId>
	for Pallet<T, I>
{
	// Check the amount approved to be spent by an owner to a delegate
	fn allowance(
		asset: T::AssetId,
		owner: &<T as SystemConfig>::AccountId,
		delegate: &<T as SystemConfig>::AccountId,
	) -> T::Balance {
		Approvals::<T, I>::get((asset, &owner, &delegate))
			.map(|x| x.amount)
			.unwrap_or_else(Zero::zero)
	}
}

impl<T: Config<I>, I: 'static> fungibles::approvals::Mutate<<T as SystemConfig>::AccountId>
	for Pallet<T, I>
{
	fn approve(
		asset: T::AssetId,
		owner: &<T as SystemConfig>::AccountId,
		delegate: &<T as SystemConfig>::AccountId,
		amount: T::Balance,
	) -> DispatchResult {
		Self::do_approve_transfer(asset, owner, delegate, amount)
	}

	// Aprove spending tokens from a given account
	fn transfer_from(
		asset: T::AssetId,
		owner: &<T as SystemConfig>::AccountId,
		delegate: &<T as SystemConfig>::AccountId,
		dest: &<T as SystemConfig>::AccountId,
		amount: T::Balance,
	) -> DispatchResult {
		Self::do_transfer_approved(asset, owner, delegate, dest, amount)
	}
}

impl<T: Config<I>, I: 'static> fungibles::roles::Inspect<<T as SystemConfig>::AccountId>
	for Pallet<T, I>
{
	fn owner(_asset: T::AssetId) -> Option<<T as SystemConfig>::AccountId> {
		None
	}

	fn issuer(_asset: T::AssetId) -> Option<<T as SystemConfig>::AccountId> {
		None
	}

	fn admin(_asset: T::AssetId) -> Option<<T as SystemConfig>::AccountId> {
		None
	}

	fn freezer(_asset: T::AssetId) -> Option<<T as SystemConfig>::AccountId> {
		None
	}
}

impl<T: Config<I>, I: 'static> fungibles::InspectEnumerable<T::AccountId> for Pallet<T, I> {
	type AssetsIterator = KeyPrefixIterator<<T as Config<I>>::AssetId>;

	/// Returns an iterator of the assets in existence.
	///
	/// NOTE: iterating this list invokes a storage read per item.
	fn asset_ids() -> Self::AssetsIterator {
		Asset::<T, I>::iter_keys()
	}
}
