mod tests {
    
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
    // TODO 为解决如何使用cargo expand 测试文件，应该是工具有bug。
    #[test]
    fn test_auto_map() {
        let person = Person {
            name: String::from("Alice"),
            age: 30,
        };
    
        let dto: PersonDto = person.into(); // 使用自动生成的 From 实现进行转换
        println!("dto: name:{}, age:{}", dto.name, dto.age);
        assert_eq!(dto.age, 30);
        assert_eq!(dto.name, "Alice");
    }
}
