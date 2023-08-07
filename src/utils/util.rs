use loan_macros::{GetStructFields,FieldsToVec};
use loan_trait::{GetStructFieldsTrait,ToVecTrait};

pub fn print_fields<T: GetStructFieldsTrait>() -> Vec<String>{
    T::get_fields()
}

pub fn to_2d_vec<T: ToVecTrait>(t:Vec<T>) ->Vec<Vec<String>>{
    let mut data:Vec<Vec<String>> = vec![];
    for item in t{
        data.push(item.to_vec());
    }
    data
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

#[test]
fn test_to_2d_vec(){
    #[derive(FieldsToVec)]
    struct MyStruct {
        field1: i32,
        field2: String,
    }

    let mystricts = vec![
        MyStruct{
            field1:1,
            field2:"1111".to_string(),
        },
        MyStruct{
            field1:2,
            field2:"2222".to_string(),
        }
    ];

    let result = to_2d_vec(mystricts);
    eprintln!("{:?}",result)
}