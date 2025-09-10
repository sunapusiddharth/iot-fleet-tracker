[package]
name = "iot-truck-agent"
version = "0.1.0"
edition = "2021"

[dependencies]
# Config
config = "0.13"
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"

# Async + Filesystem
tokio = { version = "1.3", features = ["full"] }
notify = "6.1"             # Filesystem events for hot reload
tower = "0.4"              # Watch + debounce

# Observability
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
metrics = "0.21"
metrics-exporter-prometheus = "0.12" # Optional for metrics endpoint

# Error Handling
thiserror = "1.0"
anyhow = "1.0"

# Utilities
once_cell = "1.19"         # For global config
parking_lot = "0.12"       # RwLock for config reload