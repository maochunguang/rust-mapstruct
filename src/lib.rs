extern crate proc_macro;

// 正确的mapstruct代码

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, Data, DeriveInput, Lit, Meta, NestedMeta};

#[proc_macro_attribute]
pub fn auto_map(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);

    // 提取并解析 "target" 参数
    let target_type = args
        .iter()
        .find_map(|arg| {
            if let NestedMeta::Meta(Meta::NameValue(m)) = arg {
                if m.path.is_ident("target") {
                    if let Lit::Str(lit) = &m.lit {
                        return Some(lit.value());
                    }
                }
            }
            None
        })
        .expect("auto_map requires a 'target' argument");

      // 解析输入的结构体
      let input = parse_macro_input!(input as DeriveInput);
      let struct_name = input.ident;
  
      let struct_data = match input.data {
          Data::Struct(data) => data,
          _ => panic!("auto_map only supports structs"),
      };
  
      // 提取字段名称和类型
      let (field_names, field_mappings): (Vec<_>, Vec<_>) = struct_data.fields.iter().map(|f| {
          let field_name = f.ident.as_ref().unwrap();
          let field_type = &f.ty;
          (field_name.clone(), quote! { #field_name: #field_type })
      }).unzip();
  
      // 解析目标类型
      let target_type_tokens = syn::parse_str::<syn::Type>(&target_type).unwrap();
  
      // 重新生成原始结构体和转换实现
      let expanded = quote! {
          pub struct #struct_name {
              #( #field_mappings, )*
          }
  
          impl From<#struct_name> for #target_type_tokens {
              fn from(item: #struct_name) -> #target_type_tokens {
                  #target_type_tokens {
                      #( #field_names: item.#field_names, )*
                  }
              }
          }
      };
  
      expanded.into()
}
