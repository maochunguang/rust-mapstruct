// extern crate proc_macro;

// use proc_macro::TokenStream;
// use quote::quote;
// use syn::{parse_macro_input, DeriveInput, Data, Fields, Lit, Meta, NestedMeta, AttributeArgs};

// #[proc_macro_attribute]
// pub fn auto_map(args: TokenStream, input: TokenStream) -> TokenStream {
//     // 解析宏的参数来获取目标类型
//     let args = parse_macro_input!(args as AttributeArgs);
//     let target_type = args.iter().find_map(|arg| {
//         if let NestedMeta::Meta(Meta::NameValue(m)) = arg {
//             if m.path.is_ident("target") {
//                 if let Lit::Str(lit) = &m.lit {
//                     return Some(lit.value());
//                 }
//             }
//         }
//         None
//     }).expect("auto_map requires a 'target' argument");

//     // 解析输入的结构体
//     let input = parse_macro_input!(input as DeriveInput);
//     let struct_name = input.ident;

//     let struct_data = match input.data {
//         Data::Struct(data) => data,
//         _ => panic!("auto_map only supports structs"),
//     };

//     let fields = match struct_data.fields {
//         Fields::Named(fields) => fields,
//         _ => panic!("auto_map only supports structs with named fields"),
//     };

//     // 提取字段名称
//     let field_names = fields.named.iter().map(|f| f.ident.as_ref().unwrap());

//     // 解析目标类型
//     let target_type_tokens = syn::parse_str::<syn::Type>(&target_type).unwrap();

//     // 生成转换实现
//     let expanded = if struct_name != target_type {
//         quote! {
//             impl From<#struct_name> for #target_type_tokens {
//                 fn from(item: #struct_name) -> #target_type_tokens {
//                     #target_type_tokens {
//                       #(#field_names: item.#field_names,)*
//                     }
//                 }
//             }
//         }
//     } else {
//         quote!()
//     };

//     expanded.into()
// }