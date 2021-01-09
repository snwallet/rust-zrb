use std::io::Write;
use std::io::Read;
use std::fs::OpenOptions;
// use std::fs;

fn main(){
    //File可以用来描述和实现一个文件，
    //有相关的成员变量和函数用于表示程序可以对文件进行的操作和可用的操作方法
    //所有的操作方法都会返回一个Result枚举

    //open()方法可以以只读的形式打开一个文件，如果文件不存在，抛出错误，
    //文件不可读也会抛出错误,打开文件之后会返回一个句柄

    let file = std::fs::File::open("data.txt").unwrap();
    println!("文件打开成功: {:?}", file);

    //create()方法可以返回创建的文件句柄，失败则抛出错误，
    let file1 = std::fs::File::create("data.txt").expect("create failed");
    println!("文件创建成功: {:?}", file1);

    //write_all()方法可以写入文件，用于向输出流写入内容，文件流也是输出流的一种
    let mut file2 = std::fs::File::create("data.txt").expect("create failed");
    file2.write_all("some data".as_bytes()).expect("write failed");
    file2.write_all("\nsome data".as_bytes()).expect("write failed");
    println!("data written to file");

    //读取文件，先使用open()函数打开一个文件，然后用read_to_string()将内容转换为字符串
    let mut file3 = std::fs::File::open("data.txt").unwrap();
    let mut content = String::new();
    file3.read_to_string(&mut content).unwrap();
    print!("{}", content);

    //append()追加内容，函数append()在模块std::fs::OpenOptions中定义
    let mut file4 = OpenOptions::new().append(true).open("data.txt").expect("cannot open file");
    file4.write_all("\nsome data".as_bytes()).expect("write failed");
    file4.write_all("\n学无止境".as_bytes()).expect("write failed");
    file4.write_all("\n学无止境".as_bytes()).expect("write failed");
    println!("\n数据追加成功");
    
    //remove_file()删除文件，在std::fs标准库中，就算返回Ok也可能是没有删除
    // fs::remove_file("data.txt").expect("cannot remove file");
    // println!("file is removed");



    //无copy功能自己实现
    let mut command_line: std::env::Args = std::env::args();
    command_line.next().unwrap();//跳过程序名

    //命令行参数上的源文件
    let source = command_line.next().unwrap();
    //命令行参数上的目标文件夹
    let destination = command_line.next().unwrap();

    let mut file_in = std::fs::File::open(source).unwrap();
    let mut file_out = std::fs::File::create(destination).unwrap();
    let mut buffer = [0u8; 4096];

    loop {
        let nbytes = file_in.read(&mut buffer).unwrap();
        file_out.write(&buffer[..nbytes]).unwrap();
        if nbytes < buffer.len() {
            break;
        }
    }
}