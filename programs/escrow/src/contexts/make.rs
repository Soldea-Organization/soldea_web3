use anchor_lang::prelude::*;
use anchor_spl::associated_token::*;

use anchor_spl::token::*;

use crate::state::Escrow;


#[derive(Accounts)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker : Signer<'info>,

    #[account(mut,associated_token::authority= maker,associated_token::mint = maker_mint)]
    pub maker_ata: Account<'info,TokenAccount>,
    pub maker_mint : Account<'info,Mint>,
    pub taker_mint : Account<'info,Mint>,

    #[account(seeds = [b"auth"],bump)]
    pub authority: UncheckedAccount<'info>,
    
    #[account(init,payer = maker, seeds = [b"vault",escrow.key().as_ref()],bump, token::mint=maker_mint,token::authority = escrow)]
    pub vault: Account<'info,TokenAccount>,

    #[account(init,payer = maker, seeds = [b"escrow",escrow.key().as_ref()],bump,space=Escrow::INIT_SPACE)]
    pub escrow :Account<'info,Escrow>,
    pub token_program:Program<'info, Token>,
    pub assosiated_token_program : Program<'info, AssociatedToken>,
    pub system_program : Program<'info,System>


}


impl <'info> Make <'info>{
    pub fn transfer_vault(&self, amount: u64) -> Result<()> {
        let accts= Transfer{
            from:self.maker_ata.to_account_info(),
            to:self.vault.to_account_info(),
            authority:self.maker.to_account_info(),

        };

        let cpi_ctx=CpiContext::new(self.token_program.to_account_info(),accts);
        transfer(cpi_ctx, amount)

    }

    pub fn init_escrow(&mut self, amount: u64,  seed: u64,
        escrow_bump: u8,  auth_bump: u8,
        vault_bump: u8
        ) -> Result<()>{
        self.escrow.set_inner(Escrow{
            maker: self.maker.key(),
            maker_mint: self.maker_mint.key(),
            taker_mint: self.taker_mint.key(),
            offer_amount : amount,
            auth_bump ,
            vault_bump,
            escrow_bump,
            seed,
            
        });
        Ok(())
    }
}
