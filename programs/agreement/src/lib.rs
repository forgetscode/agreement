use anchor_lang::prelude::*;


declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod agreement {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, amount_guranteed: u8, amount_total: u8) -> Result<()> {
        if amount_guranteed > amount_total {
            return Err(AgreementError::GuranteeExceededTotal.into());
        }
        ctx.accounts.contract.contractor = ctx.accounts.contractor.key();
        ctx.accounts.contract.amount_guranteed = amount_guranteed;
        ctx.accounts.contract.amount_total = amount_total;
        ctx.accounts.contract.state = ContractState::Initialized;
        Ok(())
    }

    pub fn update_amount(ctx: Context<UpdateAmount>, amount_guranteed: u8, amount_total: u8) -> Result<()> {
        match ctx.accounts.contract.state {
            ContractState::Initialized |
            ContractState::Open        |
            ContractState::OpenTo      => (),
            _ => return Err(AgreementError::ImmutableState.into()),
        }
        if ctx.accounts.contractor.key() != ctx.accounts.contract.contractor.key() {
             return Err(AgreementError::InvalidAccount.into());
         }
        ctx.accounts.contract.amount_guranteed = amount_guranteed;
        ctx.accounts.contract.amount_total = amount_total;
        Ok(())
    }

    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        match ctx.accounts.contract.state {
            ContractState::Initialized |
            ContractState::Open        |
            ContractState::OpenTo      => (),
            _ => return Err(AgreementError::ImmutableState.into()),
        }
        if ctx.accounts.destination.key() != ctx.accounts.contract.contractor.key() {
            return Err(AgreementError::InvalidAccount.into());
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = contractor, space = 8 + Contract::MAXIMUM_SIZE)]
    pub contract: Account<'info, Contract>,
    #[account(mut)]
    pub contractor: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct UpdateAmount<'info> {
    pub contract:  Account<'info, Contract>,
    pub contractor: Signer<'info>,
}

#[derive(Accounts)]
pub struct Cancel<'info> {
    #[account(mut, close = destination)]
    pub contract:  Account<'info, Contract>,
    pub destination: Signer<'info>,
}


impl Contract {
    pub const MAXIMUM_SIZE: usize = 32 + 32 + 1 + 1 + (1 + 1);

}


#[account]
pub struct Contract {
    contractor: Pubkey,     //32
    contractee: Pubkey,     //32
    amount_guranteed:  u8,  //1
    amount_total: u8,       //1
    state: ContractState,   //1 + 1
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ContractState {
    Initialized,
    Open, 
    OpenTo,
    Accepeted,
    Disputed,
    Completed,
    Cancelled,
}


#[error_code]
pub enum AgreementError {
    GuranteeExceededTotal,
    InvalidAccount,
    ImmutableState,
}
