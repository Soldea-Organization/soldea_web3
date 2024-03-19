use anchor_lang::prelude::*;
use anchor_spl::associated_token::*;

use anchor_spl::token::*;

use crate::state::Escrow;


#[derive(Accounts)]
pub struct Make<'info> {

    #[account(mut)]
    pub entrepreneur : Signer<'info>,

    #[account(mut,associated_token::authority= entrepreneur,associated_token::mint = entrepreneur_mint)]
    pub entrepreneur_ata: Account<'info,TokenAccount>,
    pub entrepreneur_mint : Account<'info,Mint>,
    pub investor_mint : Account<'info,Mint>,

    /// CHECK: This field is unsafe because...
    #[account(seeds = [b"auth"],bump)]
    pub authority: UncheckedAccount<'info>,
    
    #[account(init,payer = entrepreneur, seeds = [b"vault",escrow.key().as_ref()],bump, token::mint=entrepreneur_mint,token::authority = escrow)]
    pub vault: Account<'info,TokenAccount>,

    #[account(init,payer = entrepreneur, seeds = [b"escrow",escrow.key().as_ref()],bump,space=Escrow::INIT_SPACE)]
    pub escrow :Account<'info,Escrow>,
    pub token_program:Program<'info, Token>,
    pub assosiated_token_program : Program<'info, AssociatedToken>,
    pub system_program : Program<'info,System>


}


impl <'info> Make <'info>{
    pub fn transfer_vault(&self, amount: u64) -> Result<()> {
        let accts= Transfer{
            from:self.entrepreneur_ata.to_account_info(),
            to:self.vault.to_account_info(),
            authority:self.entrepreneur.to_account_info(),

        };

        let cpi_ctx=CpiContext::new(self.token_program.to_account_info(),accts);
        transfer(cpi_ctx, amount)

    }

    pub fn init_escrow(&mut self, amount: u64,  seed: u64,
        escrow_bump: u8,  auth_bump: u8,
        vault_bump: u8
        ) -> Result<()>{
        self.escrow.set_inner(Escrow{
            entrepreneur: self.entrepreneur.key(),
            entrepreneur_mint: self.entrepreneur_mint.key(),
            investor_mint: self.investor_mint.key(),
            offer_amount : amount,
            auth_bump ,
            vault_bump,
            escrow_bump,
            seed,
        });
        Ok(())
    }
}