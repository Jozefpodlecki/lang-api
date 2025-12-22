#![allow(dead_code)]

use std::env;
use std::net::SocketAddr;
use axum_server::tls_rustls::RustlsConfig;
use log::*;
use tokio::net::TcpListener;
use tokio::{signal};
use anyhow::{bail, Result};

use crate::data::seed_data;
use crate::{routes::*, state::AppState};

mod routes;
mod data;
mod models;
mod state;
mod utils;

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            debug!("triggered ctrl+c");
        },
        _ = terminate => {},
    }
}

fn configure_logging() -> Result<()> {
    let is_debug = cfg!(debug_assertions);

    if env::var_os("RUST_LOG").is_none() {
        if is_debug {
            unsafe { env::set_var("RUST_LOG", "debug"); }
        } else {
            unsafe { env::set_var("RUST_LOG", "info"); }
        }
    }

    flexi_logger::Logger::try_with_env()?
        .log_to_stdout()
        .start()?;

    Ok(())
}

async fn run() -> Result<()> {
    configure_logging()?;

    seed_data().await?;

    let state = AppState::new().await?;
    let router = setup_routing(state);    

    let addr: SocketAddr = option_env!("ADDR").unwrap_or_else(|| "0.0.0.0:3000")
        .parse()
        .expect("invalid ADDR");;
    // let listener = TcpListener::bind(addr).await?;
    // let addr = SocketAddr::from(addr);
    debug!("Addr: {}", addr);

    // let config = RustlsConfig::from_pem_file(
    //     "cert.pem",
    //     "key.pem",
    // )
    // .await
    // .unwrap();

    // axum::serve(listener, router.into_make_service())
    //     .with_graceful_shutdown(shutdown_signal())
    //     .await?;

    // axum_server::tls_rustls::bind_rustls(addr, config)
    axum_server::bind(addr)
        .serve(router.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[tokio::main]
async fn main() {
    match run().await {
        Ok(_) => {
            debug!("Shutdown");
        },
        Err(err) => {
            error!("{err}");
        },
    }
}