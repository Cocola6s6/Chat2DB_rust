use serde::{Serialize, Deserialize};
use serde_json::{Result, json};

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
    son: Son,
}

#[derive(Serialize, Deserialize)]
struct Son {
    name: String,
    age: u32,
}

fn main() -> Result<()> {
    // 创建一个Person对象
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        son: Son { name: "A".to_string(), age: 1 },
    };

    // 将Person对象转换为JSON字符串
    let json = serde_json::to_string(&person)?;
    let json_2 = json!(&person);

    println!("{}", json);
    println!("{}", json_2);

    Ok(())
}