pub mod vga;

pub trait Sink {
    fn putchar(&mut self, s: char);
    fn putstr(&mut self, s: &str);
}
