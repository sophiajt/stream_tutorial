mod employee;
use employee::{Employee, EmployeeRecords};

mod ided;
use ided::use_ided;

mod student;
use student::Student;

fn main() {
    let mut employee_records = EmployeeRecords::new();
    employee_records.push(Employee::new("Jane".to_string(), 101));
    employee_records.push(Employee::new("Joe".to_string(), 102));
    println!("{:#?}", employee_records);

    for employee in employee_records {
        println!("{:?}", employee);
    }

    // let employee = Employee::new(String::from("Employee"), 100);
    // let student = Student { student_id: 300 };

    // use_ided(Box::new(employee));
    // use_ided(Box::new(student));

    // println!(
    //     "employee => name: {} id: {}",
    //     employee.name(),
    //     employee.id()
    // );
}
