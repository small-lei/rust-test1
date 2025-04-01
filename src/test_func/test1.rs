

fn divide(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err("Division by zero is not allowed".to_string())
    } else {
        Ok(x / y)
    }
}


pub fn test1() {
    let x = 10.0;
    let y = 2.0;

    match divide(x, y) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}

struct TextHolder<'a> {
    content: &'a str,
}

impl <'a> TextHolder<'a> {
    fn new(text: &'a str) -> TextHolder<'a> {
        TextHolder { content: text}
    }

    fn display(&self) {
        println!("内容是：{}", self.content);
    }
}

pub fn test_str() {
    let text = String::from("这是一段shi li");
    let holder = TextHolder::new(&text);
    holder.display();
}

pub fn test_static() {
    let static_str: &'static str = "我会一直存在到程序结束";
    println!("{}", static_str);
}

pub fn test_slice() {
    // 测试while let与for的性能差异
    let v: Vec<_> = (0..1_000_000).collect();

    let mut sum: i64 = 0;
    let mut iter = v.iter();
    
    let start = std::time::Instant::now();  // 开始计时
    while let Some(&n) = iter.next() {
        sum += n;
    }
    let duration = start.elapsed();  // 获取耗时
    println!("耗时: {:?}", duration);
}