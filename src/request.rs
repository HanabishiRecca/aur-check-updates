use crate::types::Arr;
use curl::Error;
use curl::easy::{Easy, HttpVersion};
use std::io::Read;
use std::time::Duration;

#[inline(never)]
pub fn send(url: &str, body: &[u8], timeout: u64) -> Result<Arr<u8>, Error> {
    let mut easy = Easy::new();
    easy.url(url)?;
    easy.post(true)?;
    easy.http_version(HttpVersion::V11)?;
    easy.accept_encoding("")?;
    easy.fail_on_error(true)?;
    easy.tcp_nodelay(true)?;
    easy.timeout(Duration::from_millis(timeout))?;
    easy.useragent(concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")))?;
    easy.post_field_size(body.len() as u64)?;

    let mut cursor = body;
    let mut result = Vec::new();
    let mut transfer = easy.transfer();

    transfer.read_function(move |buf| Ok(cursor.read(buf).unwrap_or(0)))?;

    transfer.write_function(|data| {
        result.extend_from_slice(data);
        Ok(data.len())
    })?;

    transfer.perform()?;
    drop(transfer);

    Ok(Arr::from(result))
}
