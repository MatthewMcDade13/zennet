#[tokio::main]
async fn main() -> anyhow::Result<()> {
    zennet::http1::get("http://google.com").await?;
    Ok(())
}
