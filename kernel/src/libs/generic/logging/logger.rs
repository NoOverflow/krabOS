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

// TODO: Can a macro generate macros ?
// macro_metavar_expr
#[macro_export]
macro_rules! info {
    ($($args:tt)*) => {
        unsafe {
            match &mut KERNEL_CONTEXT.logger {
                Some(logger) => {
                    write!(logger, "[krabOS | Info] ").unwrap();
                    writeln!(logger, $($args)*).unwrap()
                },
                None => panic!(),
            }
        }
    };
}
