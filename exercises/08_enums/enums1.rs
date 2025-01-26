#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Resize,       // 不带参数，表示调整大小
    Move,         // 不带参数，表示移动
    Echo,         // 不带参数，表示回显消息
    ChangeColor,  // 不带参数，表示更改颜色
    Quit,         // 不带参数，表示退出
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}
