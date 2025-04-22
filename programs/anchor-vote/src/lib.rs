#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub use instructions::*;

declare_id!("57GyqxdH8989xw5rUcLrZze75eH7qDXomv9TYdEZeQkB");

#[program]
pub mod anchor_vote {

    use super::*;

    pub fn init_dao(ctx: Context<InitDao>, name: String) -> Result<()> {
        ctx.accounts.init_dao(name, &ctx.bumps)
    }

    pub fn init_proposal(ctx: Context<InitProposal>, proposal_name: String) -> Result<()> {
        ctx.accounts.init_proposal(proposal_name, &ctx.bumps)
    }

    pub fn vote(ctx: Context<CastVote>, vote_type: u8) -> Result<()> {
        ctx.accounts.cast_vote(vote_type, &ctx.bumps)
    }
}
