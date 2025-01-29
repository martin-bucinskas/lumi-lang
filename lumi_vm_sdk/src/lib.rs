pub trait LumiVmPlugin: Send + Sync {
  /// The name of the plugin
  fn name(&self) -> &str;
  /// The version of the plugin
  fn version(&self) -> &str;
  /// A description of the plugin
  fn description(&self) -> &str;
  /// The author of the plugin
  fn author(&self) -> &str;
  /// Called when the plugin is loaded
  fn on_load(&self) -> Result<(), String>;
  /// Called when the plugin is unloaded
  fn on_unload(&self) -> Result<(), String>;
  /// A function to perform an operation when the plugin is executed
  fn on_execute(&self, code: &str) -> Result<(), String>;
  /// Called periodically to allow the plugin to respond to events
  fn on_periodic_update(&self) -> Result<(), String>;
}

pub struct LumiVmContext<'a> {
  /// The name of the VM
  pub vm_name: String,
  pub register_hook: &'a dyn Fn(&str, Box<dyn Fn() + Send + Sync>),
}
