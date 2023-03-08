use ethers::{
	contract::{abigen, Contract},
	core::types::{BlockNumber, ValueOrArray, H160},
	providers::{Provider, StreamExt, Ws},
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

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	let client = Arc::new(get_client().await);

	let contract_address = H160::from_slice(&hex::decode(CONTRACT_ADDRESS).unwrap()[..]);

	let event = Contract::event_of_type::<SwapFilter>(client)
		.address(ValueOrArray::from(contract_address))
		.from_block(BlockNumber::Latest);

	let mut stream = event.subscribe_with_meta().await?;

	while let Some(Ok((log, _))) = stream.next().await {
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

	Ok(())
}
