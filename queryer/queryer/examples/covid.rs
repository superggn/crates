use anyhow::Result;
use queryer::query;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";

    let sql = format!(
        "SELECT location name, total_cases, new_cases, total_deaths, new_deaths \
        FROM {} where new_deaths >= 500 ORDER BY new_cases DESC",
        url
    );
    println!("sql: {}", sql);
    let df1 = query(sql).await?;
    // // 使用 polars 直接请求
    // // String 没有实现 Read, 所以要用 std::io::Cursor 包裹一下子
    // // 用 polar 工具包裹一下 csv format reader 拿到 data frame
    // let data = reqwest::get(url).await?.text().await?;
    // let df = CsvReader::new(Cursor::new(data))
    //     // .infer_schema(Some(16))
    //     .finish()?;

    // // 直接使用 polars 的 filter 功能来筛选 new_deaths >= 500 的数据
    // let filtered = df.filter(&df.column("new_deaths")?.gt(500)?)?;
    println!("====================");
    println!("{:?}", df1);
    Ok(())
}
