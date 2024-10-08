use anchor_lang::prelude::*;

#[account]
pub struct Student{
    pub wallet: Pubkey, 
    pub full_name: String,
    //TODO we can isntead check their SOL amount in student account
    //to see if Student has 0001 SOL
    pub is_paid_student: bool,
    pub bump: u8,
}

//Leave anchor discrimniator 8 to be used in instructions 
//to keep it consistent with INIT_SPACE implemntation
impl Space for Student {
    const INIT_SPACE: usize = 32 + (4 + 32) + 1 + 1 ; //32 max length in bytes and 4 is to store the length of string
}