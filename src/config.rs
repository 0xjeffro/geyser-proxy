use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub grpc: GrpcConfig,
    pub commitment: Option<String>,
    pub proxy_map: Option<HashMap<String, ProxyConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    pub endpoints: Vec<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrpcConfig {
    /// Service endpoint
    pub endpoint: String,

    /// Path of a certificate authority file
    pub ca_certificate: Option<PathBuf>,

    /// Authentication token
    pub x_token: Option<String>,

    /// Apply a timeout to connecting to the uri (ms)
    pub connect_timeout_ms: Option<u64>,

    /// Sets the tower service default internal buffer size, default is 1024
    pub buffer_size: Option<usize>,

    /// Sets whether to use an adaptive flow control
    pub http2_adaptive_window: Option<bool>,

    /// Set http2 KEEP_ALIVE_INTERVAL (ms)
    pub http2_keep_alive_interval_ms: Option<u64>,

    /// Sets the max connection-level flow control for HTTP2, default is 65,535
    pub initial_connection_window_size: Option<u32>,

    /// Sets the SETTINGS_INITIAL_WINDOW_SIZE option for HTTP2 stream-level flow control, default is 65,535
    pub initial_stream_window_size: Option<u32>,

    /// Set http2 KEEP_ALIVE_TIMEOUT (ms)
    pub keep_alive_timeout_ms: Option<u64>,

    /// Set http2 KEEP_ALIVE_WHILE_IDLE
    pub keep_alive_while_idle: Option<bool>,

    /// Set whether TCP keepalive messages are enabled on accepted connections (ms)
    pub tcp_keepalive_ms: Option<u64>,

    /// Set the value of a TCP_NODELAY option for accepted connections
    pub tcp_nodelay: Option<bool>,

    /// Apply a timeout to each request (ms)
    pub timeout_ms: Option<u64>,

    /// Max message size before decoding, full blocks can be super large, default is 1GiB
    pub max_decoding_message_size: Option<usize>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            grpc: GrpcConfig::default(),
            commitment: Some("processed".to_string()),
            proxy_map: None,
        }
    }
}

impl Default for GrpcConfig {
    fn default() -> Self {
        Self {
            endpoint: "http://127.0.0.1:10000".to_string(),
            ca_certificate: None,
            x_token: None,
            connect_timeout_ms: None,
            buffer_size: None,
            http2_adaptive_window: None,
            http2_keep_alive_interval_ms: None,
            initial_connection_window_size: None,
            initial_stream_window_size: None,
            keep_alive_timeout_ms: None,
            keep_alive_while_idle: None,
            tcp_keepalive_ms: None,
            tcp_nodelay: None,
            timeout_ms: None,
            max_decoding_message_size: Some(1024 * 1024 * 1024), // 1GiB
        }
    }
}

impl Config {
    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save_to_file(&self, path: &str) -> anyhow::Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}