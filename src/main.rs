mod crypto;
mod protocol;
mod transaction;

fn main() {
    println!("SwasChain");
    println!(
        "Protocol version: {}",
        protocol::PROTOCOL_VERSION
    );
    println!("Chain ID: {}", protocol::CHAIN_ID);
    println!("Transaction module: ready");
    println!("Crypto module: ready");
}