use crate::ided::Ided;

pub struct Student {
    pub student_id: i64,
}

impl Ided for Student {
    fn my_id(&self) -> i64 {
        self.student_id
    }
}
