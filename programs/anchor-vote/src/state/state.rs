pub use anchor_lang::prelude::*;

#[account]
#[derive(Debug, InitSpace)]
pub struct Dao {
    #[max_len(30)]
    pub name: String,
    pub authority: Pubkey,
    pub proposal_count: u64,
    pub bump: u8,
}

// [b"proposal", dao_key, proposal_count]
#[account]
#[derive(Debug, InitSpace)]
pub struct Proposal {
    pub authority: Pubkey,
    #[max_len(80)]
    pub metadata: String,
    pub yes_vote_count: u64, // This is global
    pub no_vote_count: u64,  // This is global
    // pub create_key: Pubkey,  // random-hash / u8
    pub bump: u8,
}

// [b"vote", proposal_key, authority]
#[account]
#[derive(Debug, InitSpace)]
pub struct Vote {
    pub authority: Pubkey,
    // 0 => Yes
    // 1 => No
    pub vote_type: u8,
    pub vote_credits: u64, //  This is per user
    // if quandratic funding --> total_tokens used for voting
    pub bump: u8,
}
