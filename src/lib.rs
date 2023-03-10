use {
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint,
        entrypoint::ProgramResult,
        pubkey::Pubkey,
        msg,
    },
    std::{
        vec::Vec,
        collections::HashMap
    }
};


// declare and export the program's entrypoint
entrypoint!(process_instruction);

// program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    // We require to know who are all the stakeholders
    // The accumulated revenue for each stakeholder
    // let mut _revenues:HashMap<Pubkey, u128> =  HashMap::new();

   // The funds in this contract that haven't been 
   // distributed yet.
    // let mut _accumulated: u128;

    // property value
    //let mut _totalfund: u128;


    // log a message to the blockchain
    msg!("Hello, world!");


    // Create an iterator to safely reference accounts in the slice
    let account_info_iter = &mut accounts.iter();
    let minterinfo = next_account_info(account_info_iter)?;


    // gracefully exit the program
    Ok(())
}