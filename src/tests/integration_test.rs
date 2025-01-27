use blockchain_simulation::blockchain::chain::Blockchain;

#[test]
fn test_blockchain_validity() {
    let mut blockchain = Blockchain::new(2);
    blockchain.add_transaction("Test Transaction 1".to_string());
    blockchain.mine_pending_transactions();
    assert!(blockchain.is_chain_valid());
}
