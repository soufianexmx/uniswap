use ethers::{
	contract::{abigen, Contract},
	core::types::{BlockNumber, ValueOrArray, H160},
	providers::{Provider, StreamExt, Ws},
};

use std::sync::Arc;

const WEBSOCKET_INFURA_ENDPOINT: &str =
	"wss://mainnet.infura.io/ws/v3/6084046c97ed4b93a167b6bd33cc309e";

const CONTRACT_ADDRESS: &str = "5777d92f208679db4b9778590fa3cab3ac9e2168";

async fn get_client() -> Provider<Ws> {
	Provider::<Ws>::connect(WEBSOCKET_INFURA_ENDPOINT).await.unwrap()
}

abigen!(Swap, "./src/contracts/uniswap_pool_abi.json");

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	let client = Arc::new(get_client().await);

	let contract_address = H160::from_slice(&hex::decode(CONTRACT_ADDRESS).unwrap()[..]);

	let event = Contract::event_of_type::<SwapFilter>(client)
		.address(ValueOrArray::from(contract_address))
		.from_block(BlockNumber::Latest);

	let mut stream = event.subscribe_with_meta().await?;

	while let Some(Ok((log, meta))) = stream.next().await {
		println!("{log:?}");
		println!("{meta:?}")
	}

	Ok(())
}
