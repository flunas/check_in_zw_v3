use check_in_zw_v3::App;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut my_app = App::new().await;
    my_app.run().await?;

    tokio::signal::ctrl_c().await?;
    my_app.stop().await?;
    info!("程序退出!");

    Ok(())
}