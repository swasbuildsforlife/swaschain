mod protocol;

fn main() {
    println!("SwasChain protocol v{}", protocol::PROTOCOL_VERSION);
    println!("Chain ID: {}", protocol::CHAIN_ID);
}