use lumi_vm_sdk::LumiVmPlugin;

pub struct MetricsExtension;

impl LumiVmPlugin for MetricsExtension {
    fn name(&self) -> &str {
        "Metrics Extension"
    }

    fn version(&self) -> &str {
        "0.1.0"
    }

    fn description(&self) -> &str {
        "A plugin to provide metrics for the Lumi VM"
    }

    fn author(&self) -> &str {
        "Lumi Dev Team"
    }

    fn on_load(&self) -> Result<(), String> {
        Ok(())
    }

    fn on_unload(&self) -> Result<(), String> {
        Ok(())
    }

    fn on_execute(&self, code: &str) -> Result<(), String> {
        Ok(())
    }

    fn on_periodic_update(&self) -> Result<(), String> {
        Ok(())
    }
}

// Export extension as a dynamic library
#[no_mangle]
pub extern "C" fn create_extension() -> Box<dyn LumiVmPlugin> {
    Box::new(MetricsExtension)
}
