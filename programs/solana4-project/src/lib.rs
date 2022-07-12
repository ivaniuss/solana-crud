use anchor_lang::prelude::*;
// use num_derive::*;
// use num_traits::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solana4_project {
    use super::*;
    
    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
    
    pub fn create_user(_ctx: Context<CreateUser>, _name: String, _age: u64) -> Result<u64> {
        let create_user = &mut _ctx.accounts.my_account;
        create_user.users.push(User{name: _name, age: _age});
        Ok(256)
    }

    pub fn update_user(_ctx: Context<UpdateUser>, _index: u64, _name: String, _age: u64) -> Result<()> {
        let my_account = &mut _ctx.accounts.my_account;
        my_account.users[_index as usize].name = _name;
        my_account.users[_index as usize].age = _age;
        Ok(())
    }
    pub fn delete_user(_ctx: Context<DeleteUser>, _index: u64) -> Result<()> {
        let my_account = &mut _ctx.accounts.my_account;
        my_account.users.remove(_index.try_into().unwrap());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 1000)] 
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
}

#[derive(Accounts)]
pub struct UpdateUser<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
}

#[derive(Accounts)]
pub struct DeleteUser<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
}

#[account]
pub struct MyAccount {
    users: Vec<User>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Debug)]
pub struct User {
    name: String,
    age: u64,
}