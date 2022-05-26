// SPDX-License-Identifier: Apache-2.0
// This file is part of Frontier.
//
// Copyright (c) 2020 Parity Technologies (UK) Ltd.
//
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

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use codec::Decode;
use core::marker::PhantomData;
use fp_evm::{
	Context, ExitError, ExitSucceed, Precompile, PrecompileFailure, PrecompileOutput,
	PrecompileResult,
};
use frame_support::{
	dispatch::{Dispatchable, GetDispatchInfo, PostDispatchInfo},
	weights::{DispatchClass, Pays},
};
use pallet_evm::{AddressMapping, GasWeightMapping};
use pallet_credit::CreditInterface;

use sp_core::{H160,U256};

use alloc::vec::Vec;

pub struct CreditDispatch<T> {
	_marker: PhantomData<T>,
}

impl<T> Precompile for CreditDispatch<T>
where
	T: pallet_credit::Config + pallet_evm::Config,
	T::Call: Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo + Decode,
	<T::Call as Dispatchable>::Origin: From<Option<T::AccountId>>,
{
	fn execute(
		input: &[u8],
		target_gas: Option<u64>,
		context: &Context,
		_is_static: bool,
	) -> PrecompileResult {
		let origin = T::AddressMapping::into_account_id(context.caller);
        let score = pallet_credit::Pallet::<T>::get_credit_score(&origin);
        let score = U256::from(score.unwrap());
		let mut output = Vec::new();
		score.to_little_endian(&mut output);
        Ok(PrecompileOutput {
            exit_status: ExitSucceed::Returned,
            cost: 21000,
            output,
            logs: Default::default(),
        })
	}
}
