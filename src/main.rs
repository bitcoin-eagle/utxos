use bitcoincore_rpc::{Client, RpcApi};

fn main() -> anyhow::Result<()> {
    let c = Client::new(
        "localhost:8332",
        bitcoincore_rpc::Auth::UserPass("rust-utxos".into(), "o4ka4wx3i0wxar0bec2w1sm9h".into()),
    )?;
    let hash = c.get_best_block_hash()?;
    println!("best block hash: {}", hash);
    println!("block info: {:?}", c.get_block_header_info(&hash)?);
    Ok(())
}
