use std::fmt::{Display, Formatter};
use loan_macros::{GetStructFields,FieldsToVec};
use loan_trait::{GetStructFieldsTrait};
// #[test]
// fn test_macro(){
//     #[my_first_attribute_proc_macro("attr here")]
//     fn add(a:i32,b:i32) {println!("{} + {} = {}",a,b,a+b)}
//     println!("test macro")
// }

#[test]
fn test_get_struct_fields(){
    #[derive(GetStructFields)]
    struct Person {
        name: String,
        age: u32
    }

    let fields = Person::get_fields();
    for field in fields {
        println!("{}", field);
    }
}

#[test]
fn test_get_struct_value() {
    use loan_trait::ToVecTrait;
    use std::fmt::{Display, Formatter, Result};

    #[derive(FieldsToVec)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Display for Point {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "Point (x={}, y={})", self.x, self.y)
        }
    }

    #[derive(FieldsToVec)]
    struct MyStruct {
        field1: i32,
        field2: String,
        field3: bool,
        field4: Point,
    }

    let my_struct = MyStruct {
        field1: 42,
        field2: "Hello".to_string(),
        field3: true,
        field4: Point { x: 1, y: 2 },
    };

    let vec = my_struct.to_vec();

    let expected_vec = vec![
        "42".to_string(),
        "Hello".to_string(),
        "true".to_string(),
        "Point (x=1, y=2)".to_string(),
    ];

    eprintln!("{:?}",vec)
}

#[test]
fn test_get_struct_value1(){
    use loan_trait::ToVecTrait;

    #[derive(FieldsToVec)]
    struct MyStruct {
        field1: i32,
        field2: String,
    }

    let my_struct = MyStruct {
        field1: 123,
        field2: "Hello".to_string()
    };

    let array = my_struct.to_vec();

    // 打印结果
    for item in array {
        println!("{}", item);
    }
}