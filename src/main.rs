mod employee;
use employee::Employee;

mod ided;
use ided::use_ided;

mod student;
use student::Student;

fn main() {
    let employee = Employee::new(String::from("Employee"), 100);
    let student = Student { student_id: 300 };

    use_ided(Box::new(employee));
    use_ided(Box::new(student));

    // println!(
    //     "employee => name: {} id: {}",
    //     employee.name(),
    //     employee.id()
    // );
}
