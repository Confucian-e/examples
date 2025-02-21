use anchor_lang::prelude::*;

declare_id!("7Tvuosetih5XpvrfKmQeyhcmcqix8K9ijmbho4yV8e1V");

#[program]
pub mod examples {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
