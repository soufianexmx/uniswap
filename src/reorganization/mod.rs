use ethers::{
	core::types::Log,
	prelude::FilterWatcher,
	providers::{StreamExt, Ws},
};

pub enum ReorganizationState {
	BlockRemoved,
	BlockSafe,
	Error,
}

const REORGANIZATION_DEPTH: u8 = 6;

pub async fn safe_reorganization(mut stream: FilterWatcher<'_, Ws, Log>) -> ReorganizationState {
	let mut i = 0;
	loop {
		let block_log = stream.next().await.expect("coultdn't poll filterChange stream!!!");

		if block_log.removed.expect("missing block log removed field!!!") {
			if i == REORGANIZATION_DEPTH {
				break ReorganizationState::Error
			} else {
				break ReorganizationState::BlockRemoved
			}
		} else if i == REORGANIZATION_DEPTH {
			break ReorganizationState::BlockSafe
		}

		i += 1
	}
}
