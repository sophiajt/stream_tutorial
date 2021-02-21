use crate::ided::Ided;

#[derive(Debug)]
pub struct EmployeeRecords {
    idx: usize,
    employees: Vec<Employee>,
}

impl EmployeeRecords {
    pub fn new() -> EmployeeRecords {
        EmployeeRecords {
            idx: 0,
            employees: Vec::new(),
        }
    }

    pub fn push(&mut self, employee: Employee) {
        self.employees.push(employee);
    }
}

impl Iterator for EmployeeRecords {
    type Item = Employee;

    // next() is the only required method
    fn next(&mut self) -> Option<Self::Item> {
        if self.employees.len() > self.idx {
            let output = self.employees[self.idx].clone();
            self.idx += 1;
            Some(output)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
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

    fn update(&mut self) {
        self.id = 100;
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
