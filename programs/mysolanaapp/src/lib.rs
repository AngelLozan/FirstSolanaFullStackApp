use anchor_lang::prelude::*;

declare_id!("BiwApWHC96pHwZ584G9L8awxwwRYFadyn21wjdJsyUFw");

#[program]
pub mod mysolanaapp {
    use super::*;

// Making a hello-world program to count the number of times it creates a result when called from the client side.
// Rpc request handlers used to call from a client application to interact with the program.
// Three parameters for the Create structure are base_account, user and system_program. These three are denoted with the 'pub' prefix.    
    
    pub fn create(ctx: Context<Create>) -> ProgramResult { //context structure is first parameter. Describes context that will be passed in when function is called.
        let base_account = &mut ctx.accounts.base_account;
        base_account.count = 0;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.count += 1;
        Ok(())
    }
}

// Transaction instructions. Structs that define the functions above. 
// First is 
#[derive(Accounts)] //No persisted state within the program, everything is attached to accounts. An account essentially holds all of the state of a program. All data is passed by reference from the outside.
pub struct Create<'info> { // Derived the account (state) for create and it's defined by the below
    #[account(init, payer = user, space = 16 + 16)] //#[account(...)] defines constraints and instructions that are related to the proceeding account when declared. If these don't hold, then instruction won't execute. 
    pub base_account: Account<'info, BaseAccount>, // Anyone with the proper base_account can call the RPC methods
    #[account(mut)] 
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

// Transaction instructions
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

// An account that goes inside a transaction instruction
#[account]
pub struct BaseAccount {
    pub count: u64,
}