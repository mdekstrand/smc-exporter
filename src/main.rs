mod gauges;

use std::time::Duration;

use anyhow::Result;
use gauges::SMCExportGauges;
use log::*;
use macsmc::Smc;

fn main() -> Result<()> {
    pretty_env_logger::init();

    let binding = "0.0.0.0:9684".parse()?;
    let exporter = prometheus_exporter::start(binding)?;

    let mut gauges = SMCExportGauges::create()?;
    let mut smc = Smc::connect()?;
    let interval = Duration::from_millis(100);

    loop {
        let _guard = exporter.wait_duration(interval);
        info!("updating gauges");
        gauges.update(&mut smc)?;
    }
}
