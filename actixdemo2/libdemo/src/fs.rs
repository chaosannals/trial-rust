use std::{
    env,
    path::PathBuf,
    io::Result,
};

pub fn here() -> Result<PathBuf> {
    env::current_exe()
}