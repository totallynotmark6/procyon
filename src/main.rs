#![no_main]
#![no_std]

use core::time::Duration;
use log::info;
use uefi::prelude::*;

#[entry]
fn main() {
    uefi::helpers::init().unwrap();
    info!("hello :3");
    boot::stall(Duration::from_secs(10));
    Status::SUCCESS
}
