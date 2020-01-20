use std::time::{SystemTime, SystemTimeError};

pub fn get_cur_time_unix() -> Result<u64, SystemTimeError> {
    let dur = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    Ok(dur.as_secs())
}
