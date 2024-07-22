use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        signature::{read_keypair_file, Keypair, Signer},
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

    let account = Keypair::new();

    let tx = program
        .request()
        .accounts(Initialize {
            authority: authority.pubkey(),
            my_account: account.pubkey(),
            system_program: my_program::id(),
        })
        .signer(&authority)
        .send()
        .expect("Failed to send initialize account transaction");

    println!("Initialize transaction signature: {}", tx);
}

#[test]
fn test_transfer_account() {
    let (_client, program, authority) = setup_program();

    let from_account = Keypair::new();
    let to_account = Keypair::new();

    // Initialize 'from' account
    let _tx_init_from = program
        .request()
        .accounts(Initialize {
            authority: authority.pubkey(),
            my_account: from_account.pubkey(),
            system_program: my_program::id(),
        })
        .signer(&authority)
        .send()
        .expect("Failed to send initialize 'from' account transaction");

    // Initialize 'to' account
    let _tx_init_to = program
        .request()
        .accounts(Initialize {
            authority: authority.pubkey(),
            my_account: to_account.pubkey(),
            system_program: my_program::id(),
        })
        .signer(&authority)
        .send()
        .expect("Failed to send initialize 'to' account transaction");

    // Perform transfer
    let tx = program
        .request()
        .accounts(Transfer {
            authority: authority.pubkey(),
            from: from_account.pubkey(),
            to: to_account.pubkey(),
            system_program: my_program::id(),
        })
        // .args(instruction::Transfer { amount: 1 })
        .signer(&authority)
        .send()
        .expect("Failed to send transfer transaction");

    println!("Transfer transaction signature: {}", tx);
}
