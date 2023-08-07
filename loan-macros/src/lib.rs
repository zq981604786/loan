// use proc_macro::TokenStream;
// use syn::{parse_macro_input,AttributeArgs};



// #[proc_macro_attribute]
// pub fn my_first_attribute_proc_macro(attr:TokenStream,item:TokenStream) ->TokenStream{
//     eprintln!("-------attr部分-------");
//     eprintln!("{:#?}",attr);
//     eprintln!("-------item部分-------");
//     eprintln!("{:#?}",item);
//     item
// }



// ident:匹配一个标识符(identifier),如变量名、函数名等。
// expr:匹配一个表达式。
// ty:匹配一个类型。
// pat:匹配一个模式,如结构体字段。
// tt:匹配一个 token tree,可以是任意语法单元。
// item:匹配一个代码项(item),如函数、结构体等。
// block:匹配一个代码块。
// stmt:匹配一个语句。
// path:匹配一个路径。
// meta:匹配一个属性。
// lifetime:匹配一个生命周期参数。
// vis:匹配一个可见性参数,如 pub。
//
// #[macro_export]
// macro_rules! get_struct_fields {
//   ($struct:ty)=> (
//       ((eprintln!("hello {:?}", $struct)));
//
//        (let fileds = Vec::new(); fileds)
//   )
// }


// use proc_macro::TokenStream;
// use quote::{quote, ToTokens};
// use syn::{parse_macro_input, Data, DeriveInput, Fields};
//
// #[proc_macro]
// pub fn get_struct_fields(input: TokenStream) -> TokenStream {
//     let ast: DeriveInput = parse_macro_input!(input);
//
//     let struct_name = &ast.ident;
//
//     let field_names = if let Data::Struct(data_struct) = ast.data {
//         if let Fields::Named(fields) = data_struct.fields {
//             fields.named.iter().map(|field| field.ident.as_ref().unwrap().to_token_stream()).collect::<Vec<_>>()
//         } else {
//             Vec::new()
//         }
//     } else {
//         Vec::new()
//     };
//
//     let expanded = quote! {
//         impl #struct_name {
//             fn get_fields() -> Vec<&'static str> {
//                 vec![#(stringify!(#field_names)),*]
//             }
//         }
//     };
//
//     expanded.into()
// }

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};


#[proc_macro_derive(GetStructFields)]
pub fn get_struct_fields(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input);

    let struct_name = &ast.ident;

    let field_names = if let Data::Struct(data_struct) = &ast.data {
        if let Fields::Named(fields) = &data_struct.fields {
            fields.named.iter().map(|field| &field.ident).collect::<Vec<_>>()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };
    let trait_path = quote!(GetStructFieldsTrait);
    let expanded = quote! {
        impl #trait_path for #struct_name {
            fn get_fields() -> Vec<String> {
                vec![#(stringify!(#field_names).to_owned()),*]
            }
        }
    };

    expanded.into()
}

#[proc_macro_derive(FieldsToVec)]
pub fn fields_to_vec(input: TokenStream) -> TokenStream {
    // 解析输入的 TokenStream 为 DeriveInput
    let input = parse_macro_input!(input as DeriveInput);

    // 获取结构体的名称和字段
    let struct_name = input.ident;
    let fields = match input.data {
        Data::Struct(data) => {
            if let Fields::Named(fields) = data.fields {
                fields.named
            } else {
                panic!("Only named fields are supported");
            }
        }
        _ => panic!("Only structs are supported"),
    };

    // 构建转换为数组的代码
    let field_names = fields
        .iter()
        .map(|field| field.ident.as_ref().unwrap())
        .collect::<Vec<_>>();
    let field_values = field_names.iter().map(|name| {
        quote! {
            self.#name.to_string()
        }
    });

    let trait_path = quote!(ToVecTrait);
    let expanded = quote! {
        impl #trait_path for #struct_name {
             fn to_vec(&self) -> Vec<String> {
                vec![#(#field_values),*]
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(To2DVec)]
pub fn to_2d_vec(input: TokenStream) -> TokenStream {

    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let fields = match input.data {
        Data::Struct(data) => {
            if let Fields::Named(fields) = data.fields {
                fields.named
            } else {
                panic!("Only named fields supported");
            }
        },
        _ => panic!("Only structs supported"),
    };

    let field_names: Vec<_> = fields
        .iter()
        .map(|f| f.ident.as_ref().unwrap())
        .collect();

    let field_values = field_names.iter().map(|name| {
        quote! {
            vec![self.#name.to_string()]
        }
    });

    let expanded = quote! {
        impl #struct_name {
            fn to_2d_vec(&self) -> Vec<Vec<String>> {
                vec![#(&#field_values),*]
            }
        }
    };

    TokenStream::from(expanded)
}
