fn main(){
    //使用"use"导入"fmt"模块使得"fmt::Display"可用
    use std::fmt;

    //定义一个结构体，为它实现"fmt::Display"。
    //"Structure"，包含一个"i32"元素
    struct Structure(i32);

    //为了使用"{}"标记，必须手动为类型实现"fmt::Display" trait。
    impl fmt::Display for Structure {
        //这个 trait 要求 "fmt"使用与下面呢函数完全一致的函数签名
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            //仅将self 的第一个元素写入到给定的输出流 "f"。
            //结果表明操作成功或失败，注意"write!" 的用法和 "println!"很相似
            write!(f, "{}", self.0)
        }
    }
}
