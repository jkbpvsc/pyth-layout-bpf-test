use solana_program::instruction::Instruction;
use solana_program_test::ProgramTest;
use solana_sdk::{signer::Signer, transaction::Transaction};

#[tokio::test]
async fn bpf_pyth_test() {
    let mut pt = ProgramTest::default();
    pt.prefer_bpf(true);
    pt.add_program("pyth_layout_bpf_test", pyth_layout_bpf_test::ID, None);
    pt.set_compute_max_units(1_400_000);
    let mut ctx = pt.start_with_context().await;

    let invoke_instruction = Instruction {
        program_id: pyth_layout_bpf_test::ID,
        accounts: vec![],
        data: vec![],
    };

    let transaction = Transaction::new_signed_with_payer(
        &[invoke_instruction],
        Some(&ctx.payer.pubkey()),
        &[&ctx.payer],
        ctx.last_blockhash,
    );

    ctx.banks_client
        .process_transaction(transaction)
        .await
        .unwrap();
}
