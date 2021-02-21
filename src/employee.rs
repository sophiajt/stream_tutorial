use crate::ided::Ided;

pub struct Employee {
    name: String,
    id: i64,
}

impl Employee {
    pub fn new(name: String, id: i64) -> Employee {
        Employee { name, id }
    }

    fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> i64 {
        self.id
    }
}

impl Ided for Employee {
    fn my_id(&self) -> i64 {
        self.id()
    }
}
