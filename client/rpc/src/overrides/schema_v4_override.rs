use codec::Decode;
use ethereum_types::{H160, H256, U256};
use fp_rpc::{TransactionStatusV2 as TransactionStatus};
use sc_client_api::backend::{AuxStore, Backend, StateBackend, StorageProvider};
use sp_api::BlockId;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use sp_runtime::{
	traits::{BlakeTwo256, Block as BlockT},
	Permill,
};
use sp_storage::StorageKey;
use std::{marker::PhantomData, sync::Arc};

use super::{blake2_128_extend, storage_prefix_build, StorageOverride};

/// An override for runtimes that use Schema V1
pub struct SchemaV4Override<B: BlockT, C, BE> {
	client: Arc<C>,
	_marker: PhantomData<(B, BE)>,
}

impl<B: BlockT, C, BE> SchemaV4Override<B, C, BE> {
	pub fn new(client: Arc<C>) -> Self {
		Self {
			client,
			_marker: PhantomData,
		}
	}
}

impl<B, C, BE> SchemaV4Override<B, C, BE>
where
	C: StorageProvider<B, BE> + AuxStore,
	C: HeaderBackend<B> + HeaderMetadata<B, Error = BlockChainError> + 'static,
	BE: Backend<B> + 'static,
	BE::State: StateBackend<BlakeTwo256>,
	B: BlockT<Hash = H256> + Send + Sync + 'static,
	C: Send + Sync + 'static,
{
	fn query_storage<T: Decode>(&self, id: &BlockId<B>, key: &StorageKey) -> Option<T> {
		if let Ok(Some(data)) = self.client.storage(id, key) {
			if let Ok(result) = Decode::decode(&mut &data.0[..]) {
				return Some(result);
			}
		}
		None
	}
}

impl<Block, C, BE> StorageOverride<Block> for SchemaV4Override<Block, C, BE>
where
	C: StorageProvider<Block, BE>,
	C: AuxStore,
	C: HeaderBackend<Block>,
	C: HeaderMetadata<Block, Error = BlockChainError> + 'static,
	BE: Backend<Block> + 'static,
	BE::State: StateBackend<BlakeTwo256>,
	Block: BlockT<Hash = H256> + Send + Sync + 'static,
	C: Send + Sync + 'static,
{
	/// For a given account address, returns pallet_evm::AccountCodes.
	fn account_code_at(&self, block: &BlockId<Block>, address: H160) -> Option<Vec<u8>> {
		let mut key: Vec<u8> = storage_prefix_build(b"EVM", b"AccountCodes");
		key.extend(blake2_128_extend(address.as_bytes()));
		self.query_storage::<Vec<u8>>(block, &StorageKey(key))
	}

	/// For a given account address and index, returns pallet_evm::AccountStorages.
	fn storage_at(&self, block: &BlockId<Block>, address: H160, index: U256) -> Option<H256> {
		let tmp: &mut [u8; 32] = &mut [0; 32];
		index.to_big_endian(tmp);

		let mut key: Vec<u8> = storage_prefix_build(b"EVM", b"AccountStorages");
		key.extend(blake2_128_extend(address.as_bytes()));
		key.extend(blake2_128_extend(tmp));

		self.query_storage::<H256>(block, &StorageKey(key))
	}

	/// Return the current block.
	fn current_block(&self, block: &BlockId<Block>) -> Option<ethereum::BlockV2> {
		self.query_storage::<ethereum::BlockV2>(
			block,
			&StorageKey(storage_prefix_build(b"Ethereum", b"CurrentBlock")),
		)
	}

	/// Return the current receipt.
	fn current_receipts(&self, block: &BlockId<Block>) -> Option<Vec<ethereum::ReceiptV3>> {
		self.query_storage::<Vec<ethereum::ReceiptV3>>(
			block,
			&StorageKey(storage_prefix_build(b"Ethereum", b"CurrentReceipts")),
		)
	}

	/// Return the current transaction status.
	fn current_transaction_statuses(
		&self,
		block: &BlockId<Block>,
	) -> Option<Vec<TransactionStatus>> {
		self.query_storage::<Vec<TransactionStatus>>(
			block,
			&StorageKey(storage_prefix_build(
				b"Ethereum",
				b"CurrentTransactionStatuses",
			)),
		)
	}

	/// Return the base fee at the given height.
	fn base_fee(&self, block: &BlockId<Block>) -> Option<U256> {
		self.query_storage::<U256>(
			block,
			&StorageKey(storage_prefix_build(b"BaseFee", b"BaseFeePerGas")),
		)
	}

	/// Prior to eip-1559 there is no base fee.
	fn elasticity(&self, block: &BlockId<Block>) -> Option<Permill> {
		let default_elasticity = Some(Permill::from_parts(125_000));
		let elasticity = self.query_storage::<Permill>(
			block,
			&StorageKey(storage_prefix_build(b"BaseFee", b"Elasticity")),
		);
		if elasticity.is_some() {
			elasticity
		} else {
			default_elasticity
		}
	}

	fn is_eip1559(&self, _block: &BlockId<Block>) -> bool {
		true
	}
}
