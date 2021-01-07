fn main() {
    
    // println!("{subject} {verb} {object}",  object = "the lazy dog",
    //     subject="the quick brown fox", verb="jumps over");
    // println!("{number:0>width$}", number=1, width=6);
    // println!("My name is {0}, {1} {0}", "Bond","James");
    // #[allow(dead_code)]
    // struct Structure(i32);
    // println!("this struct '{}" won't print...", Structure(3));


    println!("Hello {:5}!", "x");
    println!("Hello {:1$}!", "x", 5);
    println!("{}, '{name:.*}' has 3 characters", "Hello", 3, name=1234.56);
    println!("{}, '{name:.*}' has 3 characters", "Hello", 3, name="1234.56");
    println!("{}, '{name:>8.*}' has 3 right-aligned characters", "Hello", 3, name="1234.56");
    let pi = 3.141592;
    println!("Pi is roughly {name:.*}",  3, name=pi);
    
    //后面的变量会将前面声明的变量遮蔽起来
    //有关键字let就会重新声明一个变量（注意两个变量完全不同，名字相同而已）
    //，并且前一个同名的变量会被遮蔽
    let x = "hello";
    println!("x is {}", x);
    let x = 5;
    println!("x is {}", x);
        //应用到例如改变数组的可读写为只读
        let mut v = Vec::new(); //第一个v必须是可修改的，所以用mut，创建数组的方式
        v.push(1);
        v.push(3);
        v.push(5);
        let v = v; //对变量v进行遮蔽，此时v变量变成不可修改
        for i in &v {
            println!("{}", i);
        }
        //反过来也可以
        let v = Vec::new();
        let mut v = v;
        v.push(1);
        println!("{:?}", v);//  加入":?"起到美化输出作用

        //类型推导
        let elem = 5u8;
        //由后缀可知elem的类型是u8
        let mut v = Vec::new();
        v.push(elem);
        //通过变量的类型，推导出数组的类型是Vec<u8>
        println!("{:?}", v);

        let player_score = [("jack", 20), ("jane", 23), ("jill", 18), ("john", 19)];
        let players : Vec<_> = player_score
        //players是动态数组，内部成员的类型没有指定，交给编译器去推导
        //rust允许“局部变量/全局变量”进行推导，不过函数签名的情况下不允许
        //原因在于局部变量的作用域只是在局部，全局变量必须直接初始化
        //函数签名具有全局作用域，如果使用自动推导，可能会导致某些地方的调用出现
        //参数或者返回值类型的变化
                .iter().map(|&(player, _score)| {
                    player
                }).collect();
        println!("{:?}", players);

        //类型别名
        let x : Age = 20;
        println!("20 years later: {}", grow(x, 20));

        //静态变量
        //局部变量可以等到之后再初始化，使用之前初始化即可
        
        let x;
        let y = 1_i32;
        x = 2_i32;
        println!("{} {}", x, y);
        //全局变量因为作用域是全局，必须声明的时候初始化
        static G1 : i32 = 3;
        println!("{}", G1);
        //在函数退出的时候也不会消除内存空间，程序退出才会
        //全局变量不论读写都需要用unsafe修饰
        static mut G2 : i32 = 4;
        unsafe{
            G2 = 5;
            println!("{}", G2);
        }

        //bool数据类型类似boolean
        //可以用在条件判断表达式中，if a>= b
        //char类型的大小为4字节
        // let var5 = 0x_1234_ABCD;//使用下划线提高了阅读体验  表示十六进制数
        let x : i32 = 9;
        println!("9 power 3 = {}", x.pow(3));

        //复合数据类型
        //tuple 指的是元组类型
        // let a = (1i32, false); //元组中包含两个元素，第一个是i32类型，第二个是bool类型
        // let b = ("a", 1i32,2i32));//元组中包含两个元素，第二个元素本身也是元组，它里面还包含了一个元组
        // let a = (0,); //a是一个元组，它有一个元素
        // let b = (0); //b是一个括号表达式，它是i32类型

        //元组tuple
        let p = (1i32, 2i32);
        let (a, b) = p;
        let x = p.0;
        let y = p.1;
        println!("{} {} {} {}", a, b, x, y);
        // let empty : () = ();
        //元组内部也可以一个元素都没有，类型为unit（单元类型）
        println!("size of i8 {}", std::mem::size_of::<i8>());
        println!("size of char {}", std::mem::size_of::<char>());
        println!("size of '()' {}", std::mem::size_of::<()>());

        //结构体struct与元组类似，不过元素有名字，使用类似键值对的形式
        struct Point {
            x: i32,
            y: i32
        }
        let p = Point{x: 0, y: 1};
        println!("Point is at {} {}", p.x, p.y);

        //局部变量名字和结构体成员名字一致
        let x = 10;
        let y = 20;
        let p = Point{x, y};//省略写法,等同于Point{x: x, y: y};
        println!("Point is at {} {}", p.x, p.y);

        //访问结构体内部元素，可以使用“点”加变量名的方式，也可以使用“模式匹配”
        let p = Point{x: 9, y: 8};
        let Point{x: px, y: py} = p;
        println!("Point is at {} {}", px, py);
        //名字相同可以使用简写
        let Point{x, y} = p;
        println!("Point is at {} {}", x, y);

        //元组结构体
        struct T1 {
            v: i32
        }
        struct T2 (i32);

        let v1 = T1 {v : 1};
        let v2 = T2(1);
        let v3 = T2{0 : 1};

        let i1 = v1.v;
        let i2 = v2.0;
        let i3 = v3.0;
        println!("i1 = {}, i2 = {}, i3 = {}", i1, i2, i3);

    }


    //类型别名外面的函数
    type Age = u32;
    fn grow(age: Age, year: u32) -> Age{
        age + year
    }
    
