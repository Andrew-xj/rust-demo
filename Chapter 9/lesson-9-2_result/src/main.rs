use std::fs::File;
use std::io;
use std::io::Read;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");
    
    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating file: {:?}", e),
            }
            other_error => panic!("Error opening the file: {:?}", other_error),
        },
    };

    // 闭包
    let _f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Error creating file: {:?}", error);
            })
        } else {
            panic!("Error opening file: {:?}", error);
        }
    });

    // unwrap & expect
    let _f = File::open("hello.txt").unwrap();
    let _f = File::open("hello.txt").expect("无法打开文件 hello.txt");

    // 传播错误
    let _result = read_username_from_file();
}

fn read_username_from_file() -> Result<String, io::Error> {
    /*
    // match 匹配
    let f = File::open("hello.txt");
    
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
    */

    /*
    // ? 运算符
    let f = File::open("hello.txt")?;    
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s);
    */
    
    // 链式调用
    let mut s = String::new();
    let _f = File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}