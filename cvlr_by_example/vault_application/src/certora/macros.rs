#[macro_export]
macro_rules! assume_solvency {
    ($fv_vault:expr) => {{
        cvlr::prelude::cvlr_assume!($fv_vault.shares_total <= $fv_vault.token_total);
    }};
}

#[macro_export]
macro_rules! assert_solvency {
    ($fv_vault:expr) => {{
        cvlr::prelude::cvlr_assert!($fv_vault.shares_total <= $fv_vault.token_total);
    }};
}


#[macro_export]
macro_rules! fv_vault_from_acc_info {
    ($vault_account:expr) => {{
        let mut data = $vault_account.data.borrow_mut();
        let vault: &Vault = bytemuck::from_bytes_mut(&mut data[..]);
        FvVault::from(vault)
    }};
}
