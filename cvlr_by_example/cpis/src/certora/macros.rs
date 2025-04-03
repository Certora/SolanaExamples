#[macro_export]
macro_rules! assume_solvency {
    ($fv_vault:expr) => {{
        cvlr::prelude::cvlr_assume!(NativeInt::is_u64(&$fv_vault.shares_total));
        cvlr::prelude::cvlr_assume!(NativeInt::is_u64(&$fv_vault.token_total));

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
macro_rules! get_account_balance {
    ($account:expr) => {{
        **$account.try_borrow_mut_lamports().unwrap()
    }};
}
