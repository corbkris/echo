use crate::stores::booth::BoothStore;

pub struct EchoDatabase<'a> {
    pub booths: &'a BoothStore<'a>,
}

impl<'a> EchoDatabase<'a> {
    pub fn new(booths: &'a BoothStore<'a>) -> Self {
        Self { booths }
    }
}
