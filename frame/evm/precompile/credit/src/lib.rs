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
use pallet_credit::CreditInterface;
use pallet_evm::{AddressMapping, GasWeightMapping};

use pallet_credit::Call as CreditCall;

use sp_core::{H160, U256};

use alloc::vec::Vec;

pub struct CreditDispatch<Runtime> {
	_marker: PhantomData<Runtime>,
}

impl<Runtime> Precompile for CreditDispatch<Runtime>
where
	Runtime: pallet_credit::Config + pallet_evm::Config,
	Runtime::Call: Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo + Decode,
	<Runtime::Call as Dispatchable>::Origin: From<Option<Runtime::AccountId>>,
	Runtime::Call: From<CreditCall<Runtime>>,
{
	fn execute(
		input: &[u8],
		target_gas: Option<u64>,
		context: &Context,
		_is_static: bool,
	) -> PrecompileResult {
		let origin = Runtime::AddressMapping::into_account_id(context.caller);
		let score = pallet_credit::Pallet::<Runtime>::get_credit_score(&origin);
		let score = U256::from(score.unwrap());
		let mut output = Vec::new();
		score.to_big_endian(&mut output);
		Ok(PrecompileOutput {
			exit_status: ExitSucceed::Returned,
			cost: 21000,
			output,
			logs: Default::default(),
		})
	}
}
