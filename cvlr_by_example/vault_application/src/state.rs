use bytemuck::{Pod, Zeroable};
use solana_program::pubkey::Pubkey;
use spl_pod::primitives::PodU64;

#[repr(C)]
#[derive(Debug, Clone, Pod, Copy, Zeroable)]
pub struct Vault {
    pub owner: Pubkey,
    pub shares_total: PodU64,
    pub token_total: PodU64,
}
