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
