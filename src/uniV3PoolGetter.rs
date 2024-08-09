use crate::config::Config;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
struct GraphQLQuery {
    query: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Poo {
    id: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Pools {
    pools: Vec<Poo>,
}

#[derive(Deserialize, Serialize, Debug)]
struct PoolsData {
    pools: Vec<Pool>,
}

impl PoolsData {
    pub fn new(pools: Vec<Pool>) -> Self {
        Self { pools }
    }

    pub fn save_to_file(&self, file: &str) -> std::io::Result<()> {
        let mut file = File::create(file)?;
        let serialized = serde_json::to_string_pretty(self)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Pool {
    id: String,
    token0: Token,
    token1: Token,
    liquidity: String,
    volumeUSD: String,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
struct Token {
    id: String,
    symbol: String,
    name: String,
}

#[derive(Deserialize, Debug)]
struct PoolData {
    pool: Option<Pool>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AllPools {
    pools: Vec<Pool>,
    length: usize,
}

impl AllPools {
    pub fn new(pools: Vec<Pool>, length: usize) -> Self {
        Self { pools, length }
    }

    pub fn save_to_file(&self, file: &str) -> std::io::Result<()> {
        let mut file = File::create(file)?;
        let serialized = serde_json::to_string_pretty(self)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }
}

#[derive(Deserialize, Debug)]
struct Data {
    pools: Vec<Pool>,
}

#[derive(Deserialize, Debug)]
struct GraphQLError {
    message: String,
}

#[derive(Deserialize, Debug)]
struct GraphQLResponse<T> {
    data: T,
    errors: Option<Vec<GraphQLError>>,
}

async fn fetch_pools(
    client: &Client,
    endpoint: &str,
    skip: usize,
) -> Result<Vec<Poo>, Box<dyn Error + Send + Sync>> {
    let query = GraphQLQuery {
        query: format!(
            r#"
            {{
                pools(first: 1000, skip: {}) {{
                    id
                }}
            }}
            "#,
            skip
        ),
    };
    eprint!("   whaaaat ");

    let response = client.post(endpoint).json(&query).send().await?;

    let json: GraphQLResponse<Pools> = response.json().await?;

    eprint!("   whaaaat 3");

    if let Some(errors) = json.errors {
        for error in errors {
            eprintln!("GraphQL error: {}", error.message);
        }
        return Ok(vec![]);
    }

    Ok(json.data.pools)
}

pub async fn get_pools_list(file_name: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    let config = Config::new().await;
    let endpoint = &*config.graph_url;
    let client = Client::new();
    eprint!("sncjnsdcjhsn");

    let mut all_pools = Vec::new();
    let mut skip = 0;

    loop {
        eprint!("csjnjcn");
        let pools = fetch_pools(&client, endpoint, skip).await?;
        eprint!("   nscjnsjcn   ");
        if pools.is_empty() {
            break;
        }
        all_pools.extend(pools);
        skip += 1000;
    }

    for pool in &all_pools {
        println!("Pool address: {}", pool.id);
    }

    let mut pools: Vec<Pool> = Vec::new();
    for address in &all_pools {
        if let Some(pool) = fetch_pool_details(&client, &endpoint, &*address.id).await? {
            pools.push(pool);
            eprint!("added");
        }
    }

    let pools = PoolsData::new(pools.clone());
    pools.save_to_file(file_name)?;

    println!("Fetched detailed data for {} pools", all_pools.len());
    Ok(())
}

async fn fetch_pool_details(
    client: &Client,
    endpoint: &str,
    pool_address: &str,
) -> Result<Option<Pool>, Box<dyn Error + Send + Sync>> {
    let query = GraphQLQuery {
        query: format!(
            r#"
            {{
                pool(id: "{}") {{
                    id
                    token0 {{
                        id
                        symbol
                        name
                    }}
                    token1 {{
                        id
                        symbol
                        name
                    }}
                    liquidity
                    volumeUSD
                }}
            }}
            "#,
            pool_address
        ),
    };

    let response = client.post(endpoint).json(&query).send().await?;
    let json: GraphQLResponse<PoolData> = response.json().await?;

    if let Some(errors) = json.errors {
        for error in errors {
            eprintln!("GraphQL error: {}", error.message);
        }
        return Ok(None);
    }

    Ok(json.data.pool)
}
