use anchor_lang::prelude::*;
use anchor_spl::associated_token::*;
use anchor_spl::token::*;
use crate::state::Escrow;



#[derive(Accounts)]
pub struct Take<'info> {
    /// CHECK: This field is unsafe because...
    pub entrepreneur : UncheckedAccount<'info>,
    #[account(mut)]
    pub investor: Signer<'info>,

    #[account(mut,associated_token::authority= investor, associated_token::mint =investor_mint)]
    pub investor_ata :  Account<'info,TokenAccount>,

    #[account(mut,associated_token::authority= entrepreneur, associated_token::mint =investor_mint)]
    pub entrepreneur_receive_ata : Account<'info,TokenAccount>,

    #[account(mut,associated_token::authority= investor, associated_token::mint =entrepreneur_mint)]
    pub investor_receive_ata : Account<'info,TokenAccount>,

    pub entrepreneur_mint : Account<'info,Mint>,
    pub investor_mint : Account<'info, Mint>,

    #[account(init,payer = investor, seeds = [b"vault",escrow.key().as_ref()],bump, token::mint=investor_mint,token::authority = escrow)]
    pub vault : Account<'info, TokenAccount>,

    #[account(
        mut,
        close = entrepreneur,
        has_one = entrepreneur,
        has_one = investor_mint,
        has_one = entrepreneur_mint,
        seeds =   [b"escrow",entrepreneur.key().as_ref(),escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.escrow_bump
    )]

    pub escrow :Account<'info,Escrow>,
    pub token_program:Program<'info, Token>,
    pub assosiated_token_program : Program<'info, AssociatedToken>,
    pub system_program : Program<'info,System>

}


impl <'info> Take <'info> {
    pub fn deposit_to_entrepreneur(&self) -> Result<()> {
        // self.escrow.set_inner(Escrow{
        //     investor_mint: self.investor_mint.key(),
        //     offer_amount,
        // });

        // Ok(())
    

        let transfer_accounts = TransferChecked {
            from: self.investor_receive_ata.to_account_info(),
            to: self.entrepreneur_receive_ata.to_account_info(),
            authority: self.investor.to_account_info(),
            mint: self.investor_mint.to_account_info()
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);

        transfer_checked(cpi_ctx, self.escrow.offer_amount, self.investor_mint.decimals)
    }
    pub fn close_account(&mut self) ->  Result<()> {

        let cpi_accounts= CloseAccount{
            account : self.vault.to_account_info(),
            destination : self.investor.to_account_info(),
            authority : self.escrow.to_account_info(),

        };

        let entrepreneur_key = self.entrepreneur.key();
        let seed = self.escrow.seed.to_le_bytes();

        let seeds: &[&[u8]; 4] = &[
            b"escrow",
            entrepreneur_key.as_ref(),
            seed.as_ref(),
            &[self.escrow.escrow_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx=CpiContext::new_with_signer(self.token_program.to_account_info(),cpi_accounts,signer_seeds);
        
        close_account(cpi_ctx)
    }
}