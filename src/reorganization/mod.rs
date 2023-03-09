use ethers::{
	core::types::Log,
	prelude::FilterWatcher,
	providers::{StreamExt, Ws},
};

#[derive(Debug, PartialEq, Eq)]
pub enum ReorganizationState {
	BlockRemoved,
	BlockSafe,
	Error,
}

const REORGANIZATION_DEPTH: u8 = 6;

fn check_block(i: u8, removed: bool) -> Option<ReorganizationState> {
	if removed {
		if i == REORGANIZATION_DEPTH {
			Some(ReorganizationState::Error)
		} else {
			Some(ReorganizationState::BlockRemoved)
		}
	} else if i == REORGANIZATION_DEPTH {
		Some(ReorganizationState::BlockSafe)
	} else {
		None
	}
}

pub async fn safe_reorganization(mut stream: FilterWatcher<'_, Ws, Log>) -> ReorganizationState {
	let mut i = 0;
	loop {
		let block_log = stream.next().await.expect("coultdn't poll filterChange stream!!!");

		match check_block(i, block_log.removed.expect("missing block log removed field!!!")) {
			Some(state) => break state,
			None => i += 1,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use rand::Rng;

	#[test]
	fn block_removed_before_depth() {
		let mut rng = rand::thread_rng();

		assert_eq!(
			check_block(rng.gen_range(0..REORGANIZATION_DEPTH), true),
			Some(ReorganizationState::BlockRemoved)
		);
	}

	#[test]
	fn error_block_removed_max_depth() {
		assert_eq!(check_block(REORGANIZATION_DEPTH, true), Some(ReorganizationState::Error));
	}
}
