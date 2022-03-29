use anchor_lang::prelude::*;


declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod agreement {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, amount_guranteed: u64, amount_total: u64) -> Result<()> {
        ctx.accounts.contract.contractor = ctx.accounts.contractor.key();
        ctx.accounts.contract.amount_guranteed = amount_guranteed;
        ctx.accounts.contract.amount_total = amount_total;
        ctx.accounts.contract.bump = *ctx.bumps.get("contract").unwrap();
        ctx.accounts.contract.state = ContractState::Initialized;

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.contractor.key(),
            &ctx.accounts.contract.key(),
            amount_total,
        );

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.contractor.to_account_info(),
                ctx.accounts.contract.to_account_info(),
            ],
        )?;
        
        Ok(())
    }

    pub fn update_amount(ctx: Context<Open>, amount_guranteed: u64, amount_total: u64) -> Result<()> {
        match ctx.accounts.contract.state {
            ContractState::Initialized |
            ContractState::Open        |
            ContractState::OpenTo      => (),
            _ => return Err(AgreementError::ImmutableState.into()),
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
        Ok(())
    }

    pub fn open(ctx: Context<Open>) -> Result<()> {
        match ctx.accounts.contract.state {
            ContractState::Initialized |
            ContractState::Open        |
            ContractState::OpenTo      => (),
            _ => return Err(AgreementError::ImmutableState.into()),
        }
        ctx.accounts.contract.state = ContractState::Open;
        Ok(())
    }

    pub fn open_to(ctx: Context<Open>, open_to: Pubkey) -> Result<()> {
        match ctx.accounts.contract.state {
            ContractState::Initialized |
            ContractState::Open        |
            ContractState::OpenTo      => (),
            _ => return Err(AgreementError::ImmutableState.into()),
        }
        ctx.accounts.contract.state = ContractState::OpenTo;
        ctx.accounts.contract.contractee = open_to;
        Ok(())
    }

    pub fn accept(ctx: Context<Accept>) -> Result<()> {
        match ctx.accounts.contract.state {
            ContractState::Open => 
                {
                    ctx.accounts.contract.contractee = ctx.accounts.contractee.key();
                    ctx.accounts.contract.state = ContractState::Accepeted;
                },
            ContractState::OpenTo => 
                if ctx.accounts.contract.contractee == ctx.accounts.contractee.key(){
                    ctx.accounts.contract.state = ContractState::Accepeted;
                }
                ,
            _ => return Err(AgreementError::InvalidAccount.into()),
        }
        Ok(())
    }

    pub fn complete(ctx: Context<Complete>) -> Result<()> {
        match ctx.accounts.contract.state {
            ContractState::Accepeted => (),
            _ => return Err(AgreementError::ImmutableState.into()),
        }

        **ctx.accounts.contractee.to_account_info().lamports.borrow_mut() =
        ctx.accounts.contractee.to_account_info().lamports().checked_add(ctx.accounts.contract.amount_total).unwrap();
        **ctx.accounts.contract.to_account_info().lamports.borrow_mut() =
        ctx.accounts.contract.to_account_info().lamports().checked_sub(ctx.accounts.contract.amount_total).unwrap();


        Ok(())
    }

    pub fn dispute(ctx: Context<Complete>) -> Result<()> {
        match ctx.accounts.contract.state {
            ContractState::Accepeted => (),
            _ => return Err(AgreementError::ImmutableState.into()),
        }

        **ctx.accounts.contractee.to_account_info().lamports.borrow_mut() =
        ctx.accounts.contractee.to_account_info().lamports().checked_add(ctx.accounts.contract.amount_guranteed).unwrap();
        **ctx.accounts.contract.to_account_info().lamports.borrow_mut() =
        ctx.accounts.contract.to_account_info().lamports().checked_sub(ctx.accounts.contract.amount_guranteed).unwrap();

        Ok(())
        
    }

}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = contractor,
        space = 8 + Contract::MAXIMUM_SIZE,
        constraint = contract.amount_guranteed <= contract.amount_total,
        seeds = [b"contract_acc", contractor.key().as_ref()],
        bump,
        
    )]
    pub contract: Account<'info, Contract>,
    #[account(mut)]
    pub contractor: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Open<'info> {
    #[account(
        mut,
        constraint = contractor.key() == contract.contractor.key(),
        constraint = contract.amount_guranteed <= contract.amount_total,
        seeds = [b"contract_acc", contractor.key().as_ref()],
        bump = contract.bump
    )]
    pub contract:  Account<'info, Contract>,
    pub contractor: Signer<'info>,
}

#[derive(Accounts)]
pub struct Cancel<'info> {
    #[account(
        mut, 
        constraint = contract.contractor.key() == destination.key(),
        close = destination,
        seeds = [b"contract_acc", destination.key().as_ref()],
        bump = contract.bump
    )]
    pub contract:  Account<'info, Contract>,
    pub destination: Signer<'info>,
}

#[derive(Accounts)]
pub struct Accept<'info> {
    #[account(
        mut,
        constraint = contract.contractor.key() != contractee.key(),
        seeds = [b"contract_acc", contract.contractor.key().as_ref()],
        bump = contract.bump
    )]
    pub contract:  Account<'info, Contract>,
    pub contractee: Signer<'info>,
}

#[derive(Accounts)]
pub struct Complete<'info> {
    #[account(
        mut, 
        constraint = contract.contractor.key() == destination.key(),
        constraint = contract.contractee.key() == contractee.key(),
        close = destination,
        seeds = [b"contract_acc", destination.key().as_ref()],
        bump = contract.bump
    )]
    pub contract:  Account<'info, Contract>,
    #[account(
        mut)]
    pub contractee: SystemAccount<'info>,
    pub destination: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Contract {
    contractor: Pubkey,      //32
    contractee: Pubkey,      //32
    amount_guranteed:  u64,  //8
    amount_total: u64,       //8
    state: ContractState,    //1 + 1
    bump: u8                 //1
}

impl Contract {
    pub const MAXIMUM_SIZE: usize = 32 + 32 + 1 + 8 + 8 + (1 + 1);
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ContractState {
    Initialized,
    Open, 
    OpenTo,
    Accepeted,
    Disputed,
}

#[error_code]
pub enum AgreementError {
    InvalidAccount,
    ImmutableState,
}
