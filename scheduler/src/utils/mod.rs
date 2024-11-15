pub use settings::*;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{self, EnvFilter};

mod settings;

pub fn setup_logger() {
    let other_settings = OtherSettings::from_env().expect("Failed to load other settings");
    let filter = EnvFilter::new("")
        .add_directive(
            format!("scheduler={}", other_settings.log_level.to_lowercase())
                .parse()
                .unwrap(),
        )
        .add_directive(LevelFilter::ERROR.into());

    tracing_subscriber::fmt().with_env_filter(filter).init();
}
