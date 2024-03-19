use anchor_lang::prelude::*;
use anchor_lang::prelude::Pubkey;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};

use crate::state::Escrow;

#[derive(Accounts)]
pub struct Cancel <'info> {
    #[account(mut)]
    pub entrepreneur: Signer<'info>,

    #[account(mut, close = entrepreneur, seeds = [b"escrow",entrepreneur.key().as_ref()],bump)]
    pub escrow : Account<'info, Escrow>,
    pub entrepreneur_mint: InterfaceAccount<'info, Mint>,

    pub investor_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = entrepreneur_mint,
        associated_token::authority = escrow,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = entrepreneur_mint,
        associated_token::authority = entrepreneur,
    )]
    pub entrepreneur_ata: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,
}
impl<'info> Cancel<'info> {
    // pub fn cancel(&mut self) -> Result<()> {
    //     self.tranfer_from_escrow_to_entrepreneur()?;
    //     self.close_accounts()
    // }

    pub fn tranfer_from_escrow_to_entrepreneur(&mut self) -> Result<()> {
        let entrepreneur_key = self.entrepreneur.key();
        let seed = self.escrow.seed.to_le_bytes();

        let seeds: &[&[u8]; 4] = &[
            b"escrow",
            entrepreneur_key.as_ref(),
            seed.as_ref(),
            &[self.escrow.auth_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.entrepreneur_ata.to_account_info(),
            authority: self.escrow.to_account_info(),
            mint: self.entrepreneur_mint.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer_checked(
            cpi_ctx,
            self.vault.amount,
            self.entrepreneur_mint.decimals,
        )
    }

    pub fn close_accounts(&mut self) -> Result<()> {
        let entrepreneur_key = self.entrepreneur.key();
        let seed = self.escrow.seed.to_le_bytes();

        let seeds: &[&[u8]; 4] = &[
            b"escrow",
            entrepreneur_key.as_ref(),
            seed.as_ref(),
            &[self.escrow.auth_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.entrepreneur.to_account_info(),
            authority: self.escrow.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        close_account(cpi_ctx)
    }
}

