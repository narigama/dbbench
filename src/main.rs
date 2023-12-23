pub mod orm;
pub mod query_seaorm;
pub mod query_sql;
pub mod query_sqlx;

use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub struct Args {
    #[clap(long, env = "DATABASE_URL")]
    database_url: String,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenvy::dotenv().ok();
    let args = Args::parse();

    query_seaorm::example(&args.database_url).await?;

    Ok(())
}
