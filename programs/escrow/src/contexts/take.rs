use anchor_lang::prelude::*;
use anchor_spl::associated_token::*;
use anchor_spl::token::*;
use crate::state::Escrow;



#[derive(Accounts)]
pub struct Take<'info> {
    /// CHECK: This field is unsafe because...
    pub maker : UncheckedAccount<'info>,
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(mut,associated_token::authority= taker, associated_token::mint =taker_mint)]
    pub taker_ata :  Account<'info,TokenAccount>,

    #[account(mut,associated_token::authority= maker, associated_token::mint =taker_mint)]
    pub maker_receive_ata : Account<'info,TokenAccount>,

    #[account(mut,associated_token::authority= taker, associated_token::mint =maker_mint)]
    pub taker_receive_ata : Account<'info,TokenAccount>,

    pub maker_mint : Account<'info,Mint>,
    pub taker_mint : Account<'info, Mint>,

    #[account(init,payer = taker, seeds = [b"vault",escrow.key().as_ref()],bump, token::mint=taker_mint,token::authority = escrow)]
    pub vault : Account<'info, TokenAccount>,

    #[account(
        mut,
        close = maker,
        has_one = maker,
        has_one = taker_mint,
        has_one = maker_mint,
        seeds =   [b"escrow",maker.key().as_ref(),escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.escrow_bump
    )]

    pub escrow :Account<'info,Escrow>,
    pub token_program:Program<'info, Token>,
    pub assosiated_token_program : Program<'info, AssociatedToken>,
    pub system_program : Program<'info,System>

}


impl <'info> Take <'info> {
    pub fn deposit_to_maker(&self) -> Result<()> {
        // self.escrow.set_inner(Escrow{
        //     taker_mint: self.taker_mint.key(),
        //     offer_amount,
        // });

        // Ok(())
    

        let transfer_accounts = TransferChecked {
            from: self.taker_receive_ata.to_account_info(),
            to: self.maker_receive_ata.to_account_info(),
            authority: self.taker.to_account_info(),
            mint: self.taker_mint.to_account_info()
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);

        transfer_checked(cpi_ctx, self.escrow.offer_amount, self.taker_mint.decimals)
    }
    pub fn close_account(&mut self) ->  Result<()> {

        let cpi_accounts= CloseAccount{
            account : self.vault.to_account_info(),
            destination : self.taker.to_account_info(),
            authority : self.escrow.to_account_info(),

        };

        let maker_key = self.maker.key();
        let seed = self.escrow.seed.to_le_bytes();

        let seeds: &[&[u8]; 4] = &[
            b"escrow",
            maker_key.as_ref(),
            seed.as_ref(),
            &[self.escrow.escrow_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx=CpiContext::new_with_signer(self.token_program.to_account_info(),cpi_accounts,signer_seeds);
        
        close_account(cpi_ctx)
    }
}