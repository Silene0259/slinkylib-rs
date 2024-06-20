#[derive(Debug)]
pub struct Transaction {
    sender: String,
    receiver: String,
    hash: String,
    
    signature: String, // Placeholder for cryptographic signature
}

#[derive(Debug)]
struct Block {
    transactions: Vec<Transaction>,
    previous_hash: Option<String>, // For simplicity, using Option<String> for previous hash
    hash: String, // Placeholder for block hash
}