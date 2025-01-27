mod blockchain;
mod utils;

use blockchain::chain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new(4);

    blockchain.add_transaction("Alice -> Bob: 10 BTC".to_string());
    blockchain.mine_pending_transactions();

    blockchain.add_transaction("Bob -> Charlie: 5 BTC".to_string());
    blockchain.mine_pending_transactions();

    blockchain.print_chain();

    println!("Is chain valid? {}", blockchain.is_chain_valid());

    // Tamper with the chain
    blockchain.chain[1].transactions[0] = "Tampered Transaction".to_string();
    println!("Is chain valid after tampering? {}", blockchain.is_chain_valid());
}
