use limine::memory_map::EntryType;

pub struct PageFrameAllocator {
    memory_map: &'static [&'static limine::memory_map::Entry],
    pfsize: usize,
    head: usize,
}

// TODO: Refactor to use a more efficient data structure for allocation
// such as a bitmap or linked list.
impl PageFrameAllocator {
    pub fn new(memory_map: &'static [&limine::memory_map::Entry], pfsize: usize) -> Self {
        Self {
            memory_map,
            pfsize,
            head: 0,
        }
    }

    fn mem_iter(&self) -> impl Iterator<Item = u64> {
        self.memory_map
            .iter()
            .filter(|x| {
                x.entry_type == EntryType::USABLE
                    && x.length >= self.pfsize as u64
                    && x.base > (1 << 16)
            })
            .map(|x| x.base..(x.base + x.length))
            .flat_map(|x| x.step_by(self.pfsize))
    }

    pub fn size(&self) -> usize {
        self.mem_iter().count() * self.pfsize
    }

    pub fn alloc(&mut self) -> u64 {
        let ret = self
            .mem_iter()
            .nth(self.head)
            .expect("Page frame allocator is usable memory.");

        self.head += 1;
        ret
    }
}
