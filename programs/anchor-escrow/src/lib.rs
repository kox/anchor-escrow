use anchor_lang::prelude::*;

declare_id!("4HRSMt8m3KRZLWRAJ7AAnM8366ERDZjbKVTRDazVi7pG");

pub mod state;
pub use state::*;

pub mod contexts;
pub use contexts::*;

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, amount: u64, receive: u64) -> Result<()> {
        
        ctx.accounts.save_escrow(seed, receive, &ctx.bumps)?;
        ctx.accounts.deposit_to_vault(amount)
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.transfer_to_maker()?;
        ctx.accounts.withdraw_and_close_vault()
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund_and_close_vault()
    }
}
