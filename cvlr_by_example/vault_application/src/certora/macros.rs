#[macro_export]
macro_rules! assume_solvency {
    ($vault_account:expr) => {{
        let mut data = $vault_account.data.borrow_mut();
        let vault: &mut Vault = bytemuck::from_bytes_mut(&mut data[..]);
        let shares_total: u64 = vault.shares_total.into();
        let token_total: u64 = vault.token_total.into();
        cvlr::prelude::cvlr_assume!(shares_total <= token_total);
    }};
}

#[macro_export]
macro_rules! assert_solvency {
    ($vault_account:expr) => {{
        let mut data = $vault_account.data.borrow_mut();
        let vault: &mut Vault = bytemuck::from_bytes_mut(&mut data[..]);
        let shares_total: u64 = vault.shares_total.into();
        let token_total: u64 = vault.token_total.into();
        cvlr::prelude::cvlr_assert!(shares_total <= token_total);
    }};
}
