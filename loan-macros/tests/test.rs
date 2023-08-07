// use loan_macros::{my_first_attribute_proc_macro};
use loan_macros::{GetStructFields};
use loan_trait::{GetStructFieldsTrait};
// #[test]
// fn test_macro(){
//     #[my_first_attribute_proc_macro("attr here")]
//     fn add(a:i32,b:i32) {println!("{} + {} = {}",a,b,a+b)}
//     println!("test macro")
// }

#[test]
fn test_macro1(){
    #[derive(GetStructFields)]
    struct Person {
        name: String,
        age: u32
    }

    let fields = Person::get_fields();
    for field in fields {
        println!("{}", field);
    }

    // let p = Person{name:"zz".to_string(),age:11};
    // get_struct_fields!(Person);

    // eprintln!("{:?}",fields)
}