//获取命令行输入参数
//对命令行参数进行处理
fn main(){
        let mut line1 = String::new();
        let mut line2  = String::new();
        println!("请输入第一个参数：");
        std::io::stdin().read_line(&mut line1).unwrap();
        let l1 = line1.replace("\r\n", "").parse::<f64>().unwrap();
        println!("请输入第二个参数：");
        std::io::stdin().read_line(&mut line2).unwrap();
        let l2 = line2.replace("\r\n", "").parse::<f64>().unwrap();


        //判断参数个数分别进行处理，单个参数求圆的面积，两个求矩形面积
        if l1 == 0.0 && l2 != 0.0{
            println!("{}", l2*l2*3.14);
        }else if l1 != 0.0 && l2 == 0.0 {
            println!("{}", l1*l1*3.14);
        }else if l1 != 0.0 && l2 != 0.0{
            println!("{}", l1 * l2);
        }

}

    
    
    
    

    