use solana_program::{entrypoint::ProgramResult, pubkey::Pubkey, account_info::AccountInfo};

pub struct Processor;
impl Processor{
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        todo!()
    }
}