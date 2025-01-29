// use std::io;
// use std::io::Write;
// use log::{Level, Metadata, Record};
// use russh::CryptoVec;
// 
// struct CustomLogger;
// 
// impl log::Log for CustomLogger {
//   fn enabled(&self, metadata: &Metadata) -> bool {
//     metadata.level() <= Level::Info
//   }
// 
//   fn log(&self, record: &Record) {
//     if self.enabled(record.metadata()) {
//       if let Some(stdout) = io::stdout().lock().write_all(format!("{} - {}\n", record.level(), record.args()).as_bytes()) {
//         eprintln!("Error writing to stdout: {}", stdout);
//       }
// 
//       if let Some(session) = get_ssh_session() {
//         if let Ok(data) = CryptoVec::from(format!("{} - {}\n", record.level(), record.args())) {
//           session.data(channel, data);
//         }
//       }
//     }
//   }
// 
//   fn flush(&self) {}
// }