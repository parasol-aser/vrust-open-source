use crate::native_account_data::NativeAccountData;

use lazy_static::lazy_static;
use solana_program::{
    account_info::AccountInfo, clock::Clock, entrypoint::ProgramResult, instruction::Instruction,
    program_error::ProgramError, program_stubs, pubkey::Pubkey,
};

lazy_static! {
    static ref VERBOSE: u32 = std::env::var("FUZZ_VERBOSE")
        .map(|s| s.parse())
        .ok()
        .transpose()
        .ok()
        .flatten()
        .unwrap_or(0);
}

struct TestSyscallStubs {
    unix_timestamp: Option<i64>,
}
impl program_stubs::SyscallStubs for TestSyscallStubs {
    fn sol_get_clock_sysvar(&self, var_addr: *mut u8) -> u64 {
        let clock: Option<i64> = self.unix_timestamp;
        unsafe {
            *(var_addr as *mut _ as *mut Clock) = Clock {
                unix_timestamp: clock.unwrap(),
                ..Clock::default()
            };
        }
        solana_program::entrypoint::SUCCESS
    }

    fn sol_log(&self, message: &str) {
        if *VERBOSE >= 1 {
            println!("{}", message);
        }
    }

    fn sol_invoke_signed(
        &self,
        instruction: &Instruction,
        account_infos: &[AccountInfo],
        signers_seeds: &[&[&[u8]]],
    ) -> ProgramResult {
        let mut new_account_infos = vec![];

        // mimic check for token program in accounts
        if !account_infos.iter().any(|x| *x.key == spl_token::id()) {
            return Err(ProgramError::InvalidAccountData);
        }

        for meta in instruction.accounts.iter() {
            for account_info in account_infos.iter() {
                if meta.pubkey == *account_info.key {
                    let mut new_account_info = account_info.clone();
                    for seeds in signers_seeds.iter() {
                        let signer =
                            Pubkey::create_program_address(seeds, &stable_swap::id()).unwrap();
                        if *account_info.key == signer {
                            new_account_info.is_signer = true;
                        }
                    }
                    new_account_infos.push(new_account_info);
                }
            }
        }

        spl_token::processor::Processor::process(
            &instruction.program_id,
            &new_account_infos,
            &instruction.data,
        )
    }
}

fn test_syscall_stubs(unix_timestamp: Option<i64>) {
    // only one test may run at a time
    program_stubs::set_syscall_stubs(Box::new(TestSyscallStubs { unix_timestamp }));
}

pub fn do_process_instruction_at_time(
    instruction: Instruction,
    accounts: &[AccountInfo],
    current_ts: i64,
) -> ProgramResult {
    do_process_instruction_maybe_at_time(instruction, accounts, Some(current_ts))
}

pub fn do_process_instruction(instruction: Instruction, accounts: &[AccountInfo]) -> ProgramResult {
    do_process_instruction_maybe_at_time(instruction, accounts, None)
}

fn do_process_instruction_maybe_at_time(
    instruction: Instruction,
    accounts: &[AccountInfo],
    current_ts: Option<i64>,
) -> ProgramResult {
    test_syscall_stubs(current_ts);

    // approximate the logic in the actual runtime which runs the instruction
    // and only updates accounts if the instruction is successful
    let mut account_data = accounts
        .iter()
        .map(NativeAccountData::new_from_account_info)
        .collect::<Vec<_>>();
    let account_infos = account_data
        .iter_mut()
        .map(NativeAccountData::as_account_info)
        .collect::<Vec<_>>();
    let res = if instruction.program_id == stable_swap::id() {
        stable_swap::processor::Processor::process(
            &instruction.program_id,
            &account_infos,
            &instruction.data,
        )
    } else {
        spl_token::processor::Processor::process(
            &instruction.program_id,
            &account_infos,
            &instruction.data,
        )
    };

    if res.is_ok() {
        let mut account_metas = instruction
            .accounts
            .iter()
            .zip(accounts)
            .map(|(account_meta, account)| (&account_meta.pubkey, account))
            .collect::<Vec<_>>();
        for account_info in account_infos.iter() {
            for account_meta in account_metas.iter_mut() {
                if account_info.key == account_meta.0 {
                    let account = &mut account_meta.1;
                    let mut lamports = account.lamports.borrow_mut();
                    **lamports = **account_info.lamports.borrow();
                    let mut data = account.data.borrow_mut();
                    data.clone_from_slice(*account_info.data.borrow());
                }
            }
        }
    }
    res
}
