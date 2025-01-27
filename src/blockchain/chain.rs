use super::block::Block;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pending_transactions: Vec<String>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let mut blockchain = Blockchain {
            chain: vec![],
            pending_transactions: vec![],
            difficulty,
        };
        blockchain.create_genesis_block();
        blockchain
    }

    fn create_genesis_block(&mut self) {
        let genesis_block = Block::new(0, vec!["Genesis Block".to_string()], String::new());
        self.chain.push(genesis_block);
    }

    pub fn add_transaction(&mut self, transaction: String) {
        self.pending_transactions.push(transaction);
    }

    pub fn mine_pending_transactions(&mut self) {
        let transactions = self.pending_transactions.clone();
        self.pending_transactions.clear();
        self.add_block(transactions);
    }

    fn add_block(&mut self, transactions: Vec<String>) {
        let previous_block = self.chain.last().unwrap();
        let mut new_block = Block::new(
            previous_block.index + 1,
            transactions,
            previous_block.hash.clone(),
        );
        new_block.mine_block(self.difficulty);
        self.chain.push(new_block);
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.hash != current_block.calculate_hash() {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }

    pub fn print_chain(&self) {
        for block in &self.chain {
            println!("{:?}", block);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_blockchain_validity() {
        let mut blockchain = Blockchain::new(2);
        blockchain.add_transaction("Alice -> Bob: 10 BTC".to_string());
        blockchain.mine_pending_transactions();
        assert!(blockchain.is_chain_valid());
    }

    #[test]
    fn test_tampering_detection() {
        let mut blockchain = Blockchain::new(2);
        blockchain.add_transaction("Alice -> Bob: 10 BTC".to_string());
        blockchain.mine_pending_transactions();

        // Tamper with a block
        blockchain.chain[1].transactions[0] = "Tampered Transaction".to_string();
        assert!(!blockchain.is_chain_valid());
    }
}
