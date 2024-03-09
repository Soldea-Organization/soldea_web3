use anchor_lang::prelude::*;
use anchor_spl::*;

use crate::state::Escrow;

#[derive(Accounts)]
pub struct Cancel <'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mut,
        close = maker,
        seeds = [b"escrow", maker.key().as_ref, escrow.to_le_bytes(), as_ref()],
    )]
    pub escrow : Account<'info, Escrow>,
    pub maker_mint: InterfaceAccount<'info, Mint>,

    pub taker_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = escrow,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = maker,
    )]
    pub maker_ata: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,
}

