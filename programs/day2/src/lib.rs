use anchor_lang::prelude::*;

declare_id!("633AVwz5FczY96cCwQeCdK1jW4Ck974isnzSmCaveMhS");

#[program]
pub mod day2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>,
        a:u64,
        b:u64,
        message: String) -> Result<()> {
        msg!("You sent {} and {}", a, b);
        msg!("Your message is {}", message);
        Ok(())
    }

    pub fn array(ctx: Context<Initialize>,
        array: Vec<u64>
    ) -> Result<()> {
        msg!("Your array is {:?}", array);
        Ok(())
    }

    pub fn underflow(
        ctx: Context<Initialize>,
        a: u64,
        b: u64
    ) -> Result<()> {
        msg!("Your subtraction is {}", a - b);
        Ok(())
    }

    pub fn overflow(
        ctx: Context<Initialize>,
        a: u64,
        b: u64
    ) -> Result<()> {
        let x: u64 = a.checked_sub(b).unwrap();
        msg!("Your subtractions was indeed {:?}", x);
        Ok(())
    }

    pub fn calculator(
        ctx: Context<Initialize>,
        a: u64,
        b: u64,
        sign: char
    ) -> Result<()> { 
        match sign {
            '+' => msg
    }
        
}

#[derive(Accounts)]
pub struct Initialize {}
