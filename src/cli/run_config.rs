use super::RunArgs;
use crate::config::Config;

pub(super) async fn load(args: &RunArgs) -> Config {
    let mut config = Config::load().await.unwrap_or_default();
    Config::apply_process_access_mode_override(args.access_mode);
    config.apply_access_mode_override(args.access_mode);
    config
}
