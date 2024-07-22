use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair, Signer},
        system_program,
    },
    Client, Cluster, Program,
};
use my_program::{
    accounts::{Initialize, Transfer},
    instruction,
};
use std::sync::Arc;

fn setup_program() -> (Client<Arc<Keypair>>, Program<Arc<Keypair>>, Keypair) {
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = Arc::new(read_keypair_file(&anchor_wallet).unwrap());
    let client = Client::new_with_options(
        Cluster::Localnet,
        Arc::clone(&payer),
        CommitmentConfig::confirmed(),
    );
    let program = client.program(my_program::id()).unwrap();

    (client, program, payer.insecure_clone())
}

#[test]
fn test_initialize_account() {
    let (_client, program, authority) = setup_program();

    let (user_account, _bump) = Pubkey::find_program_address(
        &[b"my_account", authority.pubkey().as_ref()],
        &my_program::id(),
    );

    let tx = program
        .request()
        .accounts(Initialize {
            authority: authority.pubkey(),
            my_account: user_account,
            system_program: system_program::id(),
        })
        .args(instruction::Initialize { amount: 10 })
        .send()
        .expect("Failed to send initialize account transaction");

    println!("Initialize transaction signature: {}", tx);
}

#[test]
fn test_transfer_account() {
    let (_client, program, authority) = setup_program();

    let (user_account, _bump) = Pubkey::find_program_address(
        &[b"my_account", authority.pubkey().as_ref()],
        &my_program::id(),
    );

    let _tx = program
        .request()
        .accounts(Initialize {
            authority: authority.pubkey(),
            my_account: user_account,
            system_program: system_program::id(),
        })
        .args(instruction::Initialize { amount: 10 })
        .send()
        .expect("Failed to send initialize account transaction");

    // Perform transfer
    let tx = program
        .request()
        .accounts(Transfer {
            authority: authority.pubkey(),
            from: user_account,
            to: user_account,
            system_program: system_program::id(),
        })
        .args(instruction::Transfer { amount: 1 })
        .signer(&authority)
        .send()
        .expect("Failed to send transfer transaction");

    println!("Transfer transaction signature: {}", tx);
}

#[test]
#[should_panic]
fn test_transfer_invalid() {
    let (_client, program, authority) = setup_program();

    let (user_account, _bump) = Pubkey::find_program_address(
        &[b"my_account", authority.pubkey().as_ref()],
        &my_program::id(),
    );

    let _tx = program
        .request()
        .accounts(Initialize {
            authority: authority.pubkey(),
            my_account: user_account,
            system_program: system_program::id(),
        })
        .args(instruction::Initialize { amount: 0 })
        .send()
        .expect("Failed to send initialize account transaction");

    // Perform transfer
    let tx = program
        .request()
        .accounts(Transfer {
            authority: authority.pubkey(),
            from: user_account,
            to: user_account,
            system_program: system_program::id(),
        })
        .args(instruction::Transfer { amount: 1 })
        .signer(&authority)
        .send()
        .expect("Failed to send transfer transaction");

    println!("Transfer transaction signature: {}", tx);
}
