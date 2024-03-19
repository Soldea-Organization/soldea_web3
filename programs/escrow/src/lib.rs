use anchor_lang::prelude::*;

pub mod contexts;
pub mod state;

use contexts::*;


declare_id!("AmY5Fts7Q2EfFoCH1obaqV8sM6qmio9CmsoEmxQ3jYfr");

#[program]
pub mod escrow {

    use super::*;

    pub fn make(ctx: Context<Make>, amount:u64, seed: u64,
        escrow_bump: u8,  auth_bump: u8,
        vault_bump: u8) -> Result<()> {
        ctx.accounts.init_escrow(amount,seed, escrow_bump, vault_bump, auth_bump);
        ctx.accounts.transfer_vault(amount);
        Ok(())

    }


    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.close_account();
        ctx.accounts.deposit_to_maker();
        Ok(())

    }

    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        ctx.accounts.tranfer_from_escrow_to_maker();
        ctx.accounts.close_accounts();
        Ok(())
    }

}

