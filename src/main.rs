use ethers::{
	contract::Contract,
	core::types::{BlockNumber, Filter, ValueOrArray, H160},
	providers::{Middleware, StreamExt},
};
use std::sync::Arc;
use uniswap::{log_transaction, reorganization, reorganization::ReorganizationState};

const CONTRACT_ADDRESS: &str = "5777d92f208679db4b9778590fa3cab3ac9e2168";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	let client = Arc::new(uniswap::get_client().await);

	let contract_address = H160::from_slice(&hex::decode(CONTRACT_ADDRESS).unwrap()[..]);

	let event = Contract::event_of_type::<uniswap::SwapFilter>(client.clone())
		.address(ValueOrArray::from(contract_address))
		.from_block(BlockNumber::Latest);

	let mut stream = event.subscribe_with_meta().await?;

	while let Some(Ok((log, meta))) = stream.next().await {
		let filter = Filter::new().from_block(meta.block_number);
		let filter_stream = client.watch(&filter).await?;

		match reorganization::safe_reorganization(filter_stream).await {
			ReorganizationState::BlockRemoved => continue,
			ReorganizationState::BlockSafe => log_transaction(&log),
			ReorganizationState::Error => panic!("reorganization error!!!"),
		}
	}

	Ok(())
}
