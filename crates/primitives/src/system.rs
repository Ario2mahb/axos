use alloy_primitives::{Address, U256};
use serde::{Deserialize, Serialize};

/// Optimism system config contract values
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct SystemConfig {
    /// Batch sender address
    pub batch_sender: Address,
    /// L2 gas limit
    pub gas_limit: U256,
    /// Fee overhead
    pub l1_fee_overhead: U256,
    /// Fee scalar
    pub l1_fee_scalar: U256,
    /// Sequencer's signer for unsafe blocks
    pub unsafe_block_signer: Address,
}