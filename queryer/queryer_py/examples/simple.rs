use anyhow::Result;
use queryer::*;

#[tokio::main]
async fn main() -> Result<()> {
    let sql = example_sql();
    println!("example_sql: {}", sql);
    let res = queryer::query(sql).await?;
    println!("res: {:?}", res);
    Ok(())
}
