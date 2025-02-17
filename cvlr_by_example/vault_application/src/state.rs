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

impl Vault {
    pub fn deposit(&mut self, tkn: u64) {
        let mut shares_total: u64 = self.shares_total.into();
        let mut token_total: u64 = self.token_total.into();
        let shares_for_user = if shares_total == token_total {
            tkn
        } else {
            mul_div_floor(tkn, token_total, token_total)
        };

        Self::mint_shares(&mut shares_total, shares_for_user);
        Self::add_token(&mut token_total, tkn);

        self.shares_total = shares_total.into();
        self.token_total = token_total.into();
    }

    pub fn withdraw(&mut self, shares: u64) {
        let mut shares_total: u64 = self.shares_total.into();
        let mut token_total: u64 = self.token_total.into();
        let tkn_for_user = if shares_total == token_total {
            shares
        } else {
            mul_div_floor(shares, token_total, shares_total)
        };

        Self::burn_shares(&mut shares_total, shares);
        Self::del_token(&mut token_total, tkn_for_user);

        self.shares_total = shares_total.into();
        self.token_total = token_total.into();
    }

    pub fn reward(&mut self, tkn: u64) {
        let mut token_total: u64 = self.token_total.into();
        Self::add_token(&mut token_total, tkn);
        self.token_total = token_total.into();
    }

    pub fn slash(&mut self, tkn: u64) {
        let mut token_total: u64 = self.token_total.into();
        Self::del_token(&mut token_total, tkn);
        self.token_total = token_total.into();
    }

    fn mint_shares(shares_total: &mut u64, shares_for_user: u64) {
        assert!(shares_for_user > 0);
        *shares_total = shares_total.checked_add(shares_for_user).unwrap();
    }

    fn burn_shares(shares_total: &mut u64, shares: u64) {
        *shares_total = shares_total.checked_sub(shares).unwrap();
    }

    fn add_token(token_total: &mut u64, tkn: u64) {
        assert!(tkn > 0);
        *token_total = token_total.checked_add(tkn).unwrap();
    }

    fn del_token(token_total: &mut u64, tkn_for_user: u64) {
        *token_total = token_total.checked_sub(tkn_for_user).unwrap();
    }
}

fn mul_div_floor(a: u64, b: u64, c: u64) -> u64 {
    (a as u128)
        .checked_mul(b as u128)
        .unwrap()
        .checked_div(c as u128)
        .unwrap()
        .try_into()
        .unwrap()
}
