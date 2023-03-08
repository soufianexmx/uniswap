use ethers::{
	contract::abigen,
	providers::{Provider, Ws},
	utils::format_units,
};

pub mod reorganization;

abigen!(Swap, "./src/contracts/uniswap_pool_abi.json");

const WEBSOCKET_INFURA_ENDPOINT: &str =
	"wss://mainnet.infura.io/ws/v3/6084046c97ed4b93a167b6bd33cc309e";

pub async fn get_client() -> Provider<Ws> {
	Provider::<Ws>::connect(WEBSOCKET_INFURA_ENDPOINT).await.unwrap()
}

pub fn log_transaction(log: &SwapFilter) {
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
