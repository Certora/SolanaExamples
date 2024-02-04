#![cfg(feature="include-anchor")]

use { 
    solana_program::account_info::AccountInfo,
    anchor_lang::prelude::{
        Signer,
    },
    borsh::{BorshDeserialize, BorshSerialize},
    crate::{
        nondet::{
            Nondet,
            nondet_with,
        }
    },
};

use crate::{CVT_nondet_usize, NoDataVec, NoResizableVec};
use anchor_lang::prelude::borsh::maybestd::io::Write;
use anchor_lang::prelude::*;


impl Nondet for Signer<'static> {
    fn nondet() -> Self {
        let acc_info = nondet_with::<AccountInfo<'static>,_>(|x| x.is_signer);
        Self::try_from(&acc_info).unwrap()
    }
}

impl<T> BorshSerialize for NoDataVec<T> {
    fn serialize<W: Write>(&self, _writer: &mut W) -> borsh::maybestd::io::Result<()> {
        Ok(())
    }
}

impl<T:Nondet> BorshDeserialize for NoDataVec<T> {
    fn deserialize(_buf: &mut &[u8]) -> borsh::maybestd::io::Result<Self> {
        let res = NoDataVec::with_len(CVT_nondet_usize()) ;
        Ok(res)
    }
}

impl<T> BorshSerialize for NoResizableVec<T> {
    fn serialize<W: Write>(&self, _writer: &mut W) -> borsh::maybestd::io::Result<()> {
        Ok(())
    }
}

/// We need to fix the capacity of the vector.
/// However, this number depends on the specific verification task.
impl<T:Nondet> BorshDeserialize for NoResizableVec<T> {
    fn deserialize(_buf: &mut &[u8]) -> borsh::maybestd::io::Result<Self> {
        let res = NoResizableVec::new(10) ;
        Ok(res)
    }
}

