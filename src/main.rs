// #![allow(unused)]

mod utils;
pub use utils::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let span = setup_forest().await;
    let _enter = span.enter();

    let handle = tokio::spawn(bg());
    worker().await?;

    let _res = tokio::try_join!(handle)?;

    Ok(())
}

#[instrument]
async fn worker() -> Result<()> {
    info!("Worker working");
    Ok(())
}

#[instrument]
async fn bg() -> Result<()> {
    for _i in 0..3 {
        tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
        info!(immediate = true, "background");
    }
    Ok(())
}
