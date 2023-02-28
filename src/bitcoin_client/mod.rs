use bitcoincore_rpc::{
    bitcoin::BlockHash, bitcoincore_rpc_json::GetBlockHeaderResult, Client, RpcApi,
};

pub fn test() -> anyhow::Result<()> {
    let hash = get_block_hash()?;
    println!("best block hash: {}", hash);
    println!("block info: {:?}", get_block_header(&hash)?);
    Ok(())
}

pub fn get_block_hash() -> anyhow::Result<BlockHash> {
    let c = Client::new(
        "localhost:8332",
        bitcoincore_rpc::Auth::UserPass("rust-utxos".into(), "o4ka4wx3i0wxar0bec2w1sm9h".into()),
    )?;
    Ok(c.get_best_block_hash()?)
}

pub fn get_block_header(hash: &BlockHash) -> anyhow::Result<GetBlockHeaderResult> {
    let c = Client::new(
        "localhost:8332",
        bitcoincore_rpc::Auth::UserPass("rust-utxos".into(), "o4ka4wx3i0wxar0bec2w1sm9h".into()),
    )?;
    Ok(c.get_block_header_info(hash)?)
}
