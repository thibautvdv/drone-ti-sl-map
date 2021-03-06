//! Texas Instruments SimpleLink SVD to bindings for Drone, an Embedded Operating System.

#![feature(str_strip)]
#![deny(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::missing_errors_doc)]

pub use anyhow::{bail, Result};

use drone_svd::{Config, Device};
use std::{collections::HashSet, env, fs::File, path::Path};

/// Generates code for register mappings.
pub fn generate_regs(pool_number: usize, pool_size: usize) -> Result<()> {
    let out_dir = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);
    let dev = svd_deserialize()?;
    let mut output = File::create(out_dir.join("svd_regs.rs"))?;
    svd_config()?.generate_regs(&mut output, dev, pool_number, pool_size)
}

/// Generates code for interrupts and register tokens struct.
pub fn generate_rest() -> Result<()> {
    let out_dir = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);
    let dev = svd_deserialize()?;
    let mut reg_output = File::create(out_dir.join("svd_reg_index.rs"))?;
    let mut int_output = File::create(out_dir.join("svd_interrupts.rs"))?;
    svd_config()?.generate_rest(&mut reg_output, &mut int_output, dev)
}

fn svd_config() -> Result<Config<'static>> {
    let mut options = Config::new("ti_sl_reg_tokens");
    Ok(options)
}

fn svd_deserialize() -> Result<Device> {
    drone_svd::rerun_if_env_changed();
    match env::var("CARGO_CFG_TI_SL_MCU")?.as_ref() {
        "cc2538" => parse_svd("cc2538sf53.svd"),
        _ => bail!("invalid `ti_sl_mcu` cfg flag"),
    }
}

fn parse_svd(path: &str) -> Result<Device> {
    drone_svd::parse(format!("{}/files/{}", env!("CARGO_MANIFEST_DIR"), path))
}
