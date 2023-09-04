pub mod configurations;
pub mod routes;
pub mod startup;
pub mod subscription;

use env_logger::{Builder, WriteStyle};
use log::LevelFilter;

pub fn build_log() {
    let filter_level = std::env::var("RUST_LOG").unwrap_or("info".to_string());
    Builder::new()
        .filter(
            None,
            filter_level
                .parse::<LevelFilter>()
                .unwrap_or(LevelFilter::Info),
        )
        .write_style(WriteStyle::Always)
        .init();
}
