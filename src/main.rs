use ethers::{
	contract::{abigen, Contract},
	core::types::{BlockNumber, Filter, Log, ValueOrArray, H160},
	prelude::FilterWatcher,
	providers::{Middleware, Provider, StreamExt, Ws},
	utils::format_units,
};
use std::sync::Arc;

const WEBSOCKET_INFURA_ENDPOINT: &str =
	"wss://mainnet.infura.io/ws/v3/6084046c97ed4b93a167b6bd33cc309e";

const CONTRACT_ADDRESS: &str = "5777d92f208679db4b9778590fa3cab3ac9e2168";

async fn get_client() -> Provider<Ws> {
	Provider::<Ws>::connect(WEBSOCKET_INFURA_ENDPOINT).await.unwrap()
}

abigen!(Swap, "./src/contracts/uniswap_pool_abi.json");

fn log_transaction(log: &SwapFilter) {
	if log.amount_0.is_positive() && log.amount_1.is_positive() {
		panic!("swap amounts are both positive, no direction!!!");
	} else {
		let dai = format_units(log.amount_0.abs(), 18).expect("couldn't format DAI!!!");
		let usdc = format_units(log.amount_1.abs(), 6).expect("couldn't format USDC!!!");

		if log.amount_0.is_positive() {
			println!("{} : {} DAI -> {} USDC : {}", log.sender, dai, usdc, log.recipient);
		} else {
			println!("{} : {} USDC -> {} DAI : {}", log.sender, usdc, dai, log.recipient);
		}
	}
}

async fn safe_reorganization(mut stream: FilterWatcher<'_, Ws, Log>, log: &SwapFilter) {
	for n in 0..=6 {
		let block_log = stream.next().await.expect("coultdn't poll filterChange stream!!!");

		if block_log.removed.expect("missing block log removed field!!!") {
			if n == 6 {
				panic!("reorganization error!!!");
			} else {
				break
			}
		} else if n == 6 {
			log_transaction(log);
		}
	}
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	let client = Arc::new(get_client().await);

	let contract_address = H160::from_slice(&hex::decode(CONTRACT_ADDRESS).unwrap()[..]);

	let event = Contract::event_of_type::<SwapFilter>(client.clone())
		.address(ValueOrArray::from(contract_address))
		.from_block(BlockNumber::Latest);

	let mut stream = event.subscribe_with_meta().await?;

	while let Some(Ok((log, meta))) = stream.next().await {
		let filter = Filter::new().from_block(meta.block_number);
		let filter_stream = client.watch(&filter).await?;

		safe_reorganization(filter_stream, &log).await
	}

	Ok(())
}
