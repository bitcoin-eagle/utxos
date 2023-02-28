use utxos::*;

fn main() -> anyhow::Result<()> {
    bitcoin_client::test()?;
    Ok(())
}
