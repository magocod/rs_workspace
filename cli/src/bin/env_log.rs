use log::{debug, error, info, warn};

fn main() {
    env_logger::init();
    error!("{}", "And error occured");
    warn!("{:#?}", "This is important");
    info!("{:?}", "Take note");
    debug!("Something weird occured: {}", "Error");
}
