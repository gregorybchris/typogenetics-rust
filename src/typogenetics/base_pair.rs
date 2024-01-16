use crate::typogenetics::Base;

pub struct BasePair {
    pub bind: Option<Base>,
    pub comp: Option<Base>,
}

impl BasePair {
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.bind, &mut self.comp);
    }

    pub fn add_comp(&mut self) {
        if let Some(bind_base) = self.bind {
            self.comp = Some(bind_base.get_complement());
        }
    }
}
