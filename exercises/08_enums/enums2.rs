#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
enum Message {
    // TODO: Define the different variants used below.
    Resize { width: u64, height: u64 },  // 结构体变体，带有宽度和高度
    Move(Point),                         // 包含一个 Point 结构体
    Echo(String),                        // 带有一个 String 类型的消息
    ChangeColor(u8, u8, u8),             // 三个 u8 类型的颜色参数
    Quit,                                // 不带参数
}

impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let messages = [
        Message::Resize {
            width: 10,
            height: 30,
        },
        Message::Move(Point { x: 10, y: 15 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
