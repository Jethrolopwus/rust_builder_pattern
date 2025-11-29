


use struct_instance::Student;


fn main() {
    let std_1 = Student::new(String::from("harry")).unwrap_or_default();
    println!("{:?}", std_1);

    let std_2 = Student::default();
    println!("{:?}", std_2);

    let std_3 = Student {

        age: 12,
        ..Default::default()
    };
     println!("{:?}", std_3);

}
