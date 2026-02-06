//! Python bindings for feedtui using PyO3
//!
//! This module provides Python-callable functions to launch the feedtui TUI application.

pub mod app;
pub mod config;
pub mod creature;
pub mod event;
pub mod feeds;
pub mod ui;

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use std::path::PathBuf;

/// Run the feedtui TUI application
///
/// Args:
///     config_path: Optional path to config file. Defaults to ~/.feedtui/config.toml
///     refresh_interval: Optional refresh interval in seconds (overrides config)
///
/// Returns:
///     None on success, raises an exception on error
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (config_path=None, refresh_interval=None))]
fn run(config_path: Option<String>, refresh_interval: Option<u64>) -> PyResult<()> {
    // Build the tokio runtime
    let runtime = tokio::runtime::Runtime::new()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

    runtime
        .block_on(async { run_app(config_path, refresh_interval).await })
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[cfg(feature = "python")]
async fn run_app(config_path: Option<String>, refresh_interval: Option<u64>) -> anyhow::Result<()> {
    // Load config from specified path or default location
    let config_path = config_path.map(PathBuf::from).unwrap_or_else(|| {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".feedtui")
            .join("config.toml")
    });

    let mut config = config::Config::load(&config_path).unwrap_or_else(|e| {
        eprintln!(
            "Warning: Could not load config from {:?}: {}",
            config_path, e
        );
        eprintln!("Using default configuration...");
        config::Config::default()
    });

    // Apply overrides
    if let Some(refresh) = refresh_interval {
        config.general.refresh_interval_secs = refresh;
    }

    // Run the app
    let mut app = app::App::new(config);
    app.run().await
}

/// Initialize a new configuration file
///
/// Args:
///     force: If True, overwrite existing config file
///
/// Returns:
///     Path to the created config file
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (force=false))]
fn init_config(force: bool) -> PyResult<String> {
    let config_dir = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".feedtui");
    let config_path = config_dir.join("config.toml");

    // Check if config already exists
    if config_path.exists() && !force {
        return Err(PyErr::new::<pyo3::exceptions::PyFileExistsError, _>(
            format!(
                "Config file already exists at: {:?}. Use force=True to overwrite.",
                config_path
            ),
        ));
    }

    // Create config directory
    std::fs::create_dir_all(&config_dir)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;

    // Create default config
    let default_config = config::Config::default();
    let config_content = toml::to_string_pretty(&default_config)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    std::fs::write(&config_path, config_content)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;

    Ok(config_path.to_string_lossy().to_string())
}

/// Get the path to the config file
#[cfg(feature = "python")]
#[pyfunction]
fn get_config_path() -> String {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".feedtui")
        .join("config.toml")
        .to_string_lossy()
        .to_string()
}

/// Get the version of feedtui
#[cfg(feature = "python")]
#[pyfunction]
fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Python module definition
#[cfg(feature = "python")]
#[pymodule]
fn feedtui(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run, m)?)?;
    m.add_function(wrap_pyfunction!(init_config, m)?)?;
    m.add_function(wrap_pyfunction!(get_config_path, m)?)?;
    m.add_function(wrap_pyfunction!(version, m)?)?;
    Ok(())
}
