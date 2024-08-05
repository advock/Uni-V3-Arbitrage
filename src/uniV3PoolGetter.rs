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

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Pool {
    id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct AllPools {
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
struct GraphQLResponse {
    data: Option<Data>,
    errors: Option<Vec<GraphQLError>>,
}

#[derive(Deserialize, Debug)]
struct GraphQLError {
    message: String,
}

async fn fetch_pools(
    client: &Client,
    endpoint: &str,
    skip: usize,
) -> Result<Vec<Pool>, Box<dyn Error>> {
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

    let response = client.post(endpoint).json(&query).send().await?;
    let json: GraphQLResponse = response.json().await?;

    if let Some(errors) = json.errors {
        for error in errors {
            eprintln!("GraphQL error: {}", error.message);
        }
        return Ok(vec![]);
    }

    Ok(json.data.unwrap_or(Data { pools: vec![] }).pools)
}

pub async fn get_pools_list(file_name: &str) -> std::io::Result<()> {
    let config = Config::new().await;
    let endpoint: &str = &config.graph_url;
    let client = Client::new();

    let mut all_pools = Vec::new();
    let mut skip = 0;

    loop {
        let pools = fetch_pools(&client, endpoint, skip).await.unwrap();
        if pools.is_empty() {
            break;
        }
        all_pools.extend(pools);
        skip += 1000;
    }

    for pool in &all_pools {
        println!("Pool address: {}", pool.id);
    }

    let pairs = AllPools::new(all_pools.clone(), all_pools.len());

    pairs.save_to_file(file_name).unwrap();

    Ok(())
}
