use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, Transfer};

#[program]
mod dao {
    use super::*;

    #[derive(Accounts)]
    pub struct Initialize<'info> {
        #[account(init, payer = user, space = 8 + 8)]
        pub dao_account: ProgramAccount<'info, DaoAccount>,
        pub system_program: Program<'info, System>,
    }

    #[derive(Accounts)]
    pub struct Join<'info> {
        #[account(mut)]
        pub dao_account: ProgramAccount<'info, DaoAccount>,
        #[account(signer)]
        pub investor: AccountInfo<'info>,
        pub token_account: AccountInfo<'info>,
        pub token_program: AccountInfo<'info>,
    }

    pub fn join(ctx: Context<Join>) -> ProgramResult {
        let required_token_balance: u64 = 100; // Define required token balance to join
        let token_balance = token::accessor::amount(ctx.accounts.token_account)?;

        if token_balance < required_token_balance {
            return Err(ErrorCode::InsufficientTokenBalance.into());
        }

        Ok(())
    }

    #[account]
    pub struct DaoAccount {
        pub total_funds: u64,

    }


    #[error]
    pub enum ErrorCode {
        #[msg("Insufficient token balance")]
        InsufficientTokenBalance,
    }
}
