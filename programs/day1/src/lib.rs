use anchor_lang::prelude::*;

declare_id!("4SZaJAqXCLMmTrVNUeiESCyog4bJP7gGiDXc4bevUzf8");

#[program]
pub mod day1 {
    use super::*;

    pub fn initialize2(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello world");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
