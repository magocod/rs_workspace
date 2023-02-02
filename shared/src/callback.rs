type Callback = fn();

pub struct Processor {
    pub callback: Callback,
}

impl Processor {
    pub fn process_events(&self) {
        (self.callback)();
    }
}

pub struct GProcessor<CB>
where
    CB: FnMut(),
{
    pub callback: CB,
}

impl<CB> GProcessor<CB>
where
    CB: FnMut(),
{
    pub fn set_callback(&mut self, c: CB) {
        self.callback = c;
    }

    pub fn process_events(&mut self) {
        (self.callback)();
    }
}
