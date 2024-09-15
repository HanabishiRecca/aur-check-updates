use crate::types::Arr;
use curl::{easy::Easy, Error};
use std::time::Duration;

pub fn send(url: &str, timeout: u64) -> Result<Arr<u8>, Error> {
    let mut easy = Easy::new();
    easy.url(url)?;
    easy.get(true)?;
    easy.accept_encoding("")?;
    easy.fail_on_error(true)?;
    easy.tcp_nodelay(true)?;
    easy.timeout(Duration::from_millis(timeout))?;

    let mut result = Vec::new();
    let mut transfer = easy.transfer();

    transfer.write_function(|data| {
        result.extend_from_slice(data);
        Ok(data.len())
    })?;

    transfer.perform()?;
    drop(transfer);

    Ok(Arr::from(result))
}
