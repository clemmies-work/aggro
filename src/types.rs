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
pub struct Log {
    level: LogLevel,
    #[serde(rename(serialize = "_timestamp"))]
    us_since_unix_epoch: u128,

    process_id: u32,
    thread_id: u32,

    file_path: String,
    function_name: String,
    line_of_code: usize,

    content: String,
}
