fn main() {
    // let mut line = String::new();
    // println!("请输入你的名字");
    // let b = std::io::stdin().read_line(&mut line).unwrap();
    // println!("你好 {}", line);
    // println!("读取的字节数是: {}", b);

    // 命令行参数
//     let cmd_line = std::env::args();
//     println!("总共有{}个命令行参数", cmd_line.len());
//     for arg in cmd_line{
//         println!("[{}]", arg);
//     }

        let cmd_line = std::env::args();
        println!("总共有 {} 个参数", cmd_line.len());

        let mut sum = 0;
        let mut skip_first = false;
        for arg in cmd_line {
            if skip_first {
                sum += arg.parse::<i32>().unwrap();
            }
            skip_first = true;
        }
        println!("和值为:{}", sum);
}

