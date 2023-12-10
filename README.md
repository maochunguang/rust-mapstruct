# rust宏实现`mapstruct`
在java生态有个bean转换工具，叫做`mapstruct`，可以非常方便的进行bean之间的转换。原理就是可以在代码编译的时候生成转换的方法。而rust本身的宏也支持在编译的时候生成代码，因此打算用宏简单实现一个`mapstruct`。

## rust宏相关知识


## 实现原理分析
rust如果要bean之间互相转换，也很简单，可以实现`From`方法，在`From`方法里实现bean的转换赋值即可。
```rust
pub struct Person {
    name: String,
    age: u32,
}
pub struct PersonDto {
    name: String,
    age: u32,
}
impl From<Person> for PersonDto {
    fn from(item: Person) -> PersonDto {
        PersonDto {
            name: item.name,
            age: item.age,
        }
    }
}
fn main() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    let dto: PersonDto = person.into(); // 使用自动生成的 From 实现进行转换
    println!("dto: name:{}, age:{}", dto.name, dto.age);
}
```
因此如果要用rust的宏来实现，我们需要让宏来自动生成这个`From`方法，这样就可以实现自动转换。
为了使用简单，我参考了`diesel`框架的`#[diesel(table_name = blog_users)]`这种使用方法。我们的宏使用的时候直接在结构体上加上`#[auto_map(target = "PersonDto")]`就可以了，非常的简洁优雅。
```rust
#[auto_map(target = "PersonDto")]
pub struct Person {
    name: String,
    age: u32,
}
```

## 代码实现
由于宏的使用方法是`#[auto_map(target = "PersonDto")]`，因此宏的工作流程也基本确定了，以Person和PersonDto结构体为例子，大致的工作流程如下：
1. 提取宏auto_map的"target" 参数。
2. 解析输入的结构体（PersonDto）。
3. 提取输入结构体的字段名称和类型。
4. 解析目标类型。
5. 重新生成原始结构体和From方法实现。

### 第一步，创建工程，加依赖
```bash
cargo new rust_mapstruct --lib
cd rust_mapstruct
```
因为宏定义生成代码需要解析rust的ast，因此需要依赖两个关键的库，quote，syn。因为要定义宏生成代码，因此需要指定`proc-macro = true`。
整体依赖如下：
```toml
[lib]
proc-macro = true

[dependencies]
proc-macro2 = "1.0"
quote = "1.0"
syn = { version = "1.0.17", features = ["full"] }
```

### 第二步，修改lib.rs核心代码
#### 1、定义核心方法
```rust
#[proc_macro_attribute]
pub fn auto_map(args: TokenStream, input: TokenStream) -> TokenStream {

}
```
#### 2、提取并解析 "target" 参数
```rust
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
```

#### 3、解析输入的结构体（PersonDto）
```rust
      // 解析输入的结构体
      let input = parse_macro_input!(input as DeriveInput);
      let struct_name = input.ident;
  
      let struct_data = match input.data {
          Data::Struct(data) => data,
          _ => panic!("auto_map only supports structs"),
      };
```

#### 4、提取PersonDto字段名称和类型
```rust
    let (field_names, field_mappings): (Vec<_>, Vec<_>) = struct_data.fields.iter().map(|f| {
          let field_name = f.ident.as_ref().unwrap();
          let field_type = &f.ty;
          (field_name.clone(), quote! { #field_name: #field_type })
      }).unzip();
```

#### 5、解析目标类型(PersonDto)
```rust
 // 解析目标类型
      let target_type_tokens = syn::parse_str::<syn::Type>(&target_type).unwrap();
```

#### 6、生成原始结构体和From方法实现
```rust
// 重新生成原始结构体和转换实现
      let expanded = quote! {
          // 注意这里是生成原结构体Person
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
```

### 第三步，用项目测试宏
先把宏的项目编译一下，在命令行执行`cargo build`。
新创建一个测试项目，
```bash
cargo new test-mapstruct
cd test-mapstruct
```
#### 修改Cargo.toml依赖关系
```toml
[dependencies]
rust_mapstruct = { path = "../rust_mapstruct" }
```
#### 用main.rs写一个简单的测试例子
```rust
use rust_mapstruct::auto_map;

#[auto_map(target = "PersonDto")]
pub struct Person {
    name: String,
    age: u32,
}
pub struct PersonDto {
    name: String,
    age: u32,
}
fn main() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    let dto: PersonDto = person.into(); // 使用自动生成的 From 实现进行转换
    println!("dto: name:{}, age:{}", dto.name, dto.age);
}
```
#### 执行代码看成果
在test-mapstruct项目执行`cargo build`,`cargo run`，看成果！
```bash
❯ cargo build
   Compiling test-mapstruct v0.1.0 (/home/maocg/study/test-mapstruct)
    Finished dev [unoptimized + debuginfo] target(s) in 0.26s

test-mapstruct on  master 
❯ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/test-mapstruct`
dto: name:Alice, age:30
```