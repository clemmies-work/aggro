use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum LogLevel {
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
    Trace = 5,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProcessInfo {
    id: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DebugInfo {
    file_path: String,
    function_name: String,
    line_of_code: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogFlatten {
    level: LogLevel,
    us_since_unix_epoch: u128,

    #[serde(flatten)]
    process_info: ProcessInfo,
    #[serde(flatten)]
    debug_info: DebugInfo,

    content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(from="LogFlatten")]
pub struct Log {
    level: LogLevel,
    us_since_unix_epoch: u128,

    #[serde(flatten)]
    process_info: ProcessInfo,
    #[serde(flatten)]
    debug_info: DebugInfo,

    content: String,
}

impl From<LogFlatten> for Log {
    fn from(other: LogFlatten) -> Self {
        Self {
            level: other.level,
            us_since_unix_epoch: other.us_since_unix_epoch,
            process_info: other.process_info,
            debug_info: other.debug_info,
            content: other.content,
        }
    }
}
