use borsh::{BorshDeserialize, BorshSerialize};
use borsh_derive::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct UserAccount{
    pub user_no :u64, 
    pub user_address: [u8;32], 
    pub book_id:u64,
    pub borrow_date:u64, 
    pub return_date:u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct  UserCount{
    pub user_counter: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct Book{
   pub book_id:u64,
   pub total_Number_Of_Books: u64, 
   pub in_circulation: u64, 
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct  BookCount{
    pub book_counter: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct Config{
    pub config_accounts1:[u8;32],
    pub config_accounts2:[u8;32], 
    pub config_accounts3:[u8;32], 

}


