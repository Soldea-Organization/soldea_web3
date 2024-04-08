use anchor_lang::prelude::*;
use anchor_spl::token::{self,Mint, TokenAccount,Transfer};


#[program]
mod crowdfunding {
    use super::*;



    pub fn contribute(ctx: Context<Contribute>, amount: u64) -> ProgramResult {
        let crowdfunding = &mut ctx.accounts.crowdfunding;
        crowdfunding.total_funds += amount;
        Ok(())
    }

    pub fn initialize(ctx: Context<Initialize>, total_funding: u64, equity_percentage: u8) -> ProgramResult {
        let crowdfunding = &mut ctx.accounts.crowdfunding;
        crowdfunding.total_funding = total_funding;
        crowdfunding.equity_percentage = equity_percentage;
        crowdfunding.total_funds = 0;
        crowdfunding.entrepreneur = *ctx.accounts.entrepreneur.key;

        // Distribute project-specific tokens to investors
        distribute_tokens(ctx.accounts, equity_percentage)?;

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> ProgramResult {
        let crowdfunding = &mut ctx.accounts.crowdfunding;

        // Check if the request is approved by the DAO
        if *ctx.accounts.dao.key != ctx.accounts.dao_authority.key {
            return Err(ErrorCode::Unauthorized.into());
        }

        // Check if the requested amount is available
        if crowdfunding.total_funds < amount {
            return Err(ErrorCode::InsufficientFunds.into());
        }

        
        // Transfer funds to the entrepreneur
        token::transfer(ctx.accounts.into(), amount)?;

        Ok(())
    }


    fn distribute_tokens(accounts: &Initialize, equity_percentage: u8) -> ProgramResult {
        // Calculate the total token supply based on the total funding and equity percentage
        let total_token_supply = (accounts.crowdfunding.total_funding * equity_percentage as u64) / 100;

        // Distribute tokens to investors
        // Iterate over investors and transfer tokens based on their investment amount
        for investor in &accounts.investors {
            let token_amount = (investor.investment * equity_percentage as u64) / 100;
            // Transfer token_amount to investor's token account
            // Code for transferring tokens goes here...
        }

        Ok(())
    }
    


}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub crowdfunding: ProgramAccount<'info, Crowdfunding>,
    #[account(signer)]
    pub dao_authority: AccountInfo<'info>,
    pub dao: AccountInfo<'info>,
    #[account(mut)]
    pub transfer_to_entrepreneur: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub crowdfunding: ProgramAccount<'info, Crowdfunding>,
    pub entrepreneur: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Contribute<'info> {
    #[account(mut)]
    pub crowdfunding: ProgramAccount<'info, Crowdfunding>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub crowdfunding: ProgramAccount<'info, Crowdfunding>,
    #[account(signer)]
    pub dao_authority: AccountInfo<'info>,
    pub dao: AccountInfo<'info>,
    #[account(mut)]
    pub transfer_to_entrepreneur: Transfer<'info, TokenAccount<'info>>,
}


#[account]
pub struct Crowdfunding {
    pub total_funding: u64,          // Total funding requested by the entrepreneur
    pub equity_percentage: u8,       // Percentage of equity offered to investors
    pub total_funds: u64,            // Total funds collected from investors
    pub entrepreneur: Pubkey,        // Public key of the entrepreneur
    pub expiration_time: i64,         // Expiration time for the crowdfunding (Unix timestamp)

}


#[error]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Insufficient funds")]
    InsufficientFunds,
}


