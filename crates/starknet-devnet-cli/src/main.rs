use starknet_devnet_cli::rundevnet;


#[tokio::main]
async fn main()-> Result<(), anyhow::Error>{
 rundevnet().await
}