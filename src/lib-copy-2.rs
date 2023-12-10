// extern crate proc_macro;

// use proc_macro::TokenStream;
// use quote::quote;
// use syn::{parse_macro_input, AttributeArgs, DeriveInput, Lit, Meta, NestedMeta};

// #[proc_macro_attribute]
// pub fn auto_map(args: TokenStream, input: TokenStream) -> TokenStream {
//     // 解析宏的参数来获取目标类型
//     let args = parse_macro_input!(args as AttributeArgs);
//     let target_type_str = args
//         .iter()
//         .find_map(|arg| {
//             if let NestedMeta::Meta(Meta::NameValue(m)) = arg {
//                 if m.path.is_ident("target") {
//                     if let Lit::Str(lit) = &m.lit {
//                         return Some(lit.value());
//                     }
//                 }
//             }
//             None
//         })
//         .expect("auto_map requires a 'target' argument");

//     // 解析目标类型
//     let target_type = syn::parse_str::<syn::Type>(&target_type_str).unwrap();

//     // 解析输入的结构体
//     let input = parse_macro_input!(input as DeriveInput);
//     let struct_name = input.ident;

//     // 复制字段数据
//     let fields: Vec<_> = match input.data {
//         syn::Data::Struct(syn::DataStruct { fields, .. }) => fields.iter().cloned().collect(),
//         _ => panic!("auto_map only supports structs"),
//     };

//     // 提取字段名称
//     let field_names: Vec<_> = fields.iter().filter_map(|f| f.ident.as_ref()).collect();

//     // 生成转换实现
//     let expanded = quote! {
//       impl From<#struct_name> for #target_type {
//         fn from(item: #struct_name) -> #target_type {
//           #target_type {
//             #( #struct_name::#field_names: item.#field_names, )*
//           }
//         }
//       }
//     };

//     // 将目标类型作为参数传递给宏
//     let expanded = quote! {
//       #expanded (#target_type)
//     };

//     expanded.into()
// }
