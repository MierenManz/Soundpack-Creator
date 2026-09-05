use std::fmt::Debug;
use std::fmt::Display;
use std::time::SystemTime;

#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
  Debug = 1,
  Info = 2,
  Warning = 3,
  Error = 4,
  Critical = 5,
}

impl Display for LogLevel {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Debug::fmt(&self, f)
  }
}

pub fn log_fmt(level: LogLevel, msg: &str) {
  let _system_time = SystemTime::now();

  println!("[{level}]: {msg}");
}

pub fn debug(msg: &str) {
  log_fmt(LogLevel::Debug, msg);
}

pub fn info(msg: &str) {
  log_fmt(LogLevel::Info, msg);
}

pub fn warning(msg: &str) {
  log_fmt(LogLevel::Warning, msg);
}

pub fn error(msg: &str) {
  log_fmt(LogLevel::Error, msg);
}

pub fn critical(msg: &str) {
  log_fmt(LogLevel::Critical, msg);
}
