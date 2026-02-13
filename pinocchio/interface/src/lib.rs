#![no_std]

pub mod error;
pub mod instruction;
pub mod native_mint;
pub mod state;

pub mod program {
    pinocchio_pubkey::declare_id!("ixCPtHBGeiuHL8uGFvCr2Lr9LYijeBjeRbXJEK7qSPL");
}
