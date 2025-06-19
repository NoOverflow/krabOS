use core::fmt::Write;

use crate::libs::drivers::logs::sinks::Sink;

pub struct Logger<'a> {
    // TODO: Manage multiple sinks
    pub sink: &'a mut dyn crate::drivers::logs::sinks::Sink,
}

impl<'a> Logger<'a> {
    pub fn new(sink: &'a mut dyn Sink) -> Self {
        Logger { sink }
    }
}

impl<'a> Write for Logger<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.sink.putstr(s);
        Ok(())
    }
}
