use anchor_lang::prelude::*;



#[account]
pub struct Escrow{
    pub maker: Pubkey,
    pub maker_mint: Pubkey,
    pub taker_mint: Pubkey,
    pub offer_amount: u64,
    pub auth_bump: u8,
    pub vault_bump: u8,
    pub escrow_bump: u8,
    pub seed: u64

}
impl Space for Escrow {
    const INIT_SPACE: usize = 8 + 8 + 32 + 32 + 32 + 8 + 8 + 8 + 1;
}
