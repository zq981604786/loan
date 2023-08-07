use loan_macros::{GetStructFields,FieldsToVec};
use loan_trait::{GetStructFieldsTrait,ToVecTrait};

pub fn print_fields<T: GetStructFieldsTrait>() -> Vec<String>{
    T::get_fields()
}

#[test]
fn test_print_fields(){
    #[derive(GetStructFields)]
    struct Person {
        name: String,
        age: u32
    }

    // let fields = Person::get_fields();
    // for field in fields {
    //     eprintln!("{}", field);
    // }

    let result = print_fields::<Person>();
    eprintln!("{:?}",result)
}