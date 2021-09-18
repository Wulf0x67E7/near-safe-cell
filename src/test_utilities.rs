#[cfg(test)]
pub(crate) struct Buffer<'a> {
    buffer: &'a mut [u8],
    used: usize,
}
#[cfg(test)]
impl<'a> Buffer<'a> {
    pub(crate) fn new(buffer: &'a mut [u8]) -> Self {
        Buffer { buffer, used: 0 }
    }
    pub(crate) fn as_str(self) -> Option<&'a str> {
        if self.used <= self.buffer.len() {
            Some(unsafe { core::str::from_utf8_unchecked(&self.buffer[..self.used]) })
        } else {
            None
        }
    }
}
#[cfg(test)]
impl<'a> core::fmt::Write for Buffer<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        if self.used > self.buffer.len() {
            return Err(core::fmt::Error);
        }
        let remaining_buf = &mut self.buffer[self.used..];
        let raw_s = s.as_bytes();
        let write_num = core::cmp::min(raw_s.len(), remaining_buf.len());
        remaining_buf[..write_num].copy_from_slice(&raw_s[..write_num]);
        self.used += raw_s.len();
        if write_num < raw_s.len() {
            Err(core::fmt::Error)
        } else {
            Ok(())
        }
    }
}
#[cfg(test)]
pub(crate) fn format<'a>(
    buffer: &'a mut [u8],
    args: core::fmt::Arguments,
) -> core::result::Result<&'a str, core::fmt::Error> {
    let mut w = Buffer::new(buffer);
    core::fmt::write(&mut w, args)?;
    w.as_str().ok_or(core::fmt::Error)
}
