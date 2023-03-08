use ethers::{
	contract::{abigen, Contract},
	core::types::{BlockNumber, Filter, Log, ValueOrArray, H160},
	prelude::FilterWatcher,
	providers::{Middleware, Provider, StreamExt, Ws},
	utils::format_units,
};
use std::sync::Arc;

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

		uniswap::safe_reorganization(filter_stream, &log).await
	}

	Ok(())
}
