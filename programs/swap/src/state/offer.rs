use anchor_lang::prelaude::*;


#[account]
#[derive(InitSpace)]
pub struct offer {
    pub id: u64,
    pub maker: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub bump: u8,
}