use anyhow::{Context, Result};
use uefi::boot::ScopedProtocol;

pub fn get_procotol<T: uefi::proto::Protocol>() -> Result<ScopedProtocol<T>> {
    let handle = uefi::boot::get_handle_for_protocol::<T>().context("cant get simple fs handle")?;

    let protocol =
        uefi::boot::open_protocol_exclusive::<T>(handle).context("cant open simple fs protocol")?;

    return Ok(protocol);
}
