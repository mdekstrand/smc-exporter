//! Collection of gauges for SMC reporting.
use std::time::Instant;

use anyhow::Result;
use log::*;
use macsmc::Smc;
use prometheus_exporter::prometheus::{register_counter, register_gauge, Counter, Gauge};
pub struct SMCExportGauges {
    system_power: Gauge,
    system_energy: Counter,
    dc_in_power: Gauge,

    energy_time: Instant,
}

impl SMCExportGauges {
    /// Create a new set of gauges.
    pub fn create() -> Result<SMCExportGauges> {
        Ok(SMCExportGauges {
            system_power: register_gauge!(
                "smc_system_power_watts",
                "Current total system power draw"
            )?,
            system_energy: register_counter!(
                "smc_system_energy_joules",
                "Estimated system energy consumption"
            )?,
            dc_in_power: register_gauge!("smc_dc_in_power_watts", "Current DC input power")?,
            energy_time: Instant::now(),
        })
    }

    /// Update the gauges for export.
    pub fn update(&mut self, smc: &mut Smc) -> Result<()> {
        self.update_energy(smc)?;
        self.dc_in_power.set(smc.power_dc_in()?.0 as f64);

        Ok(())
    }

    /// Update system power and energy consumption.
    ///
    /// This is a separate update function so it can be run periodically to main
    /// a more precise estimate of energy consumption than we would get if we
    /// only respond to requests.
    pub fn update_energy(&mut self, smc: &mut Smc) -> Result<()> {
        let power = smc.power_system_total()?.0 as f64;
        self.system_power.set(power);
        let now = Instant::now();
        let time = now.duration_since(self.energy_time);
        debug!("{} W over {}s", power, time.as_secs_f64());
        self.system_energy.inc_by(time.as_secs_f64() * power);
        self.energy_time = now;
        Ok(())
    }
}
