use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::prelude::*;
use ethers::providers::Provider;
use std::sync::Arc;

pub struct Config {
    // Http provider
    pub http: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    // Websocket provider
    pub wss: Arc<Provider<Ws>>,
    // pub ipc: Arc<Provider<Ipc>>,
    pub wallet: Arc<Wallet<SigningKey>>,

    pub wss_log: Arc<Provider<Ws>>,
}

impl Config {
    // Implement a constructor for the configuration struct
    pub async fn new() -> Self {
        let http_url = std::env::var("NETWORK_HTTP").expect("missing NETWORK_RPC");
        let provider: Provider<Http> = Provider::<Http>::try_from(http_url).unwrap();

        let wss_url = std::env::var("NETWORK_WSS").expect("missing NETWORK_WSS");
        let ws_provider: Provider<Ws> = Provider::<Ws>::connect(wss_url).await.unwrap();

        let wss_log_url = std::env::var("NETWORK_WSS_LOGS").expect("missing new logs");
        let wsl_provider: Provider<Ws> = Provider::<Ws>::connect(wss_log_url).await.unwrap();

        let chain_id = provider.get_chainid().await.unwrap().as_u64();

        let private_key = std::env::var("PRIVATE_KEY").expect("missing PRIVATE_KEY");
        println!("Private Key: {}", private_key);

        let wallet = match private_key.parse::<LocalWallet>() {
            Ok(wallet) => wallet.with_chain_id(chain_id),
            Err(e) => {
                eprintln!("Invalid PRIVATE_KEY: {}", e);
                std::process::exit(1);
            }
        };

        let middleware = Arc::new(SignerMiddleware::new(provider, wallet.clone()));
        Self {
            http: middleware,
            wss: Arc::new(ws_provider),
            wallet: Arc::new(wallet),
            wss_log: Arc::new(wsl_provider),
        }
    }
}
