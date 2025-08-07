use crate::storage::StorageType;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Config {
    storage: StorageConfig,
    stream: StreamConfig,
    model: ModelConfig,
}

#[derive(Debug, Deserialize)]
struct StorageConfig {
    #[serde(rename = "type")]
    storage_type: String,
    local_file: LocalFileConfig,
    qdrant: QdrantConfig,
}

#[derive(Debug, Deserialize)]
struct LocalFileConfig {
    path: String,
}

#[derive(Debug, Deserialize)]
struct QdrantConfig {
    url: String,
    collection_name: String,
    api_key: Option<String>,
}

#[derive(Debug, Deserialize)]
struct StreamConfig {
    url: String,
    num_images: usize,
    interval_millis: u64,
    chunk_size: usize,
}

#[derive(Debug, Deserialize)]
struct ModelConfig {
    name: String,
    embedding_size: usize,
}

lazy_static::lazy_static! {
    static ref CONFIG: Config = load_config().expect("Failed to load configuration");
}

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_content = fs::read_to_string("config.yaml")?;
    let config: Config = serde_yaml::from_str(&config_content)?;
    Ok(config)
}

pub fn get_storage_config() -> StorageType {
    match CONFIG.storage.storage_type.as_str() {
        "local_file" => StorageType::LocalFile(CONFIG.storage.local_file.path.clone()),
        "qdrant" => StorageType::Qdrant {
            url: CONFIG.storage.qdrant.url.clone(),
            collection_name: CONFIG.storage.qdrant.collection_name.clone(),
            api_key: CONFIG.storage.qdrant.api_key.clone(),
        },
        _ => {
            eprintln!("Unknown storage type: {}, defaulting to local_file", CONFIG.storage.storage_type);
            StorageType::LocalFile("embeddings.json".to_string())
        }
    }
}

pub fn get_stream_url() -> &'static str {
    &CONFIG.stream.url
}

pub fn get_num_images() -> usize {
    CONFIG.stream.num_images
}

pub fn get_interval_millis() -> u64 {
    CONFIG.stream.interval_millis
}

pub fn get_chunk_size() -> usize {
    CONFIG.stream.chunk_size
}

pub fn get_model_name() -> &'static str {
    &CONFIG.model.name
}

pub fn get_embedding_size() -> usize {
    CONFIG.model.embedding_size
} 