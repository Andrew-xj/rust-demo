use std::collections::HashMap;

fn main() {
    // 基本操作
    let mut scores = HashMap::new();
    
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    
    match score {
        Some(s) => println!("{}", s),
        None => println!("team not exist"),
    };
    
    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }

    // 另一种创建方式
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let intial_scores = vec![10, 50];
    
    let scores: HashMap<_, _> = 
    	teams.iter().zip(intial_scores.iter()).collect();
    println!("{:#?}", scores);

    // 所有权问题
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    
    let mut map = HashMap::new();
    // map.insert(field_name, field_value);	// 所有权失效
    map.insert(&field_name, &field_value);	// 存入引用
    println!("{}: {}", field_name, field_value);

    // V 更新
    let text = "hello world wonderful world";
    
    let mut map = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("{:#?}", map);
}