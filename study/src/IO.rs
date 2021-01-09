use::std::io::Write;

fn main(){
    //IO表示如何从标准输入中读取数据并把读取的数据显示出来
    // Read特质用于从输入流读取字节的组件，包括标准输入、键盘、鼠标、命令行、文件等
    //Write特质用于向输出流中写入数据，包含字节数据和UTF-8数据
    //read_line(&mut line) -> Result，
    //用于从输入流中读取一行字符串数据并存储在line参数中，返回一个Result枚举，成功返回数据

    let mut line = String::new();
    println!("请输入你的名字");
    let b = std::io::stdin().read_line(&mut line).unwrap();
    println!("你好 {}", line);
    println!("读取的字节数是: {}", b);

    // write(&buf) -> Result 把参数buf中的全部或部分字节写入底层的流，
    //不会自己换行要手动加上去
    //返回Reuslt枚举，如果成功返回写入的字节数，日常使用的print!和println!相同功能

    let b1 = std::io::stdout().write("我的未来".as_bytes()).unwrap();
    let b2 = std::io::stdout().write("zheng1224.github.io".as_bytes()).unwrap();
    std::io::stdout().write(format!("\n写入的字节数为: {}\n", (b1+b2)).as_bytes()).unwrap();


    //命令行参数
    //多个参数之间用空格间隔，参数中间有空格就用双引号包起来
    //通过遍历std::env::args()来输出参数
    //cargo build cargo run 参数1 参数2 参数3
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