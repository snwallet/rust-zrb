use movies::play;//使用use关键字可以预先引入模块中的函数和结构体，
//从而不需要在使用的时候使用限定模块
use movie::english::comedy::play1;

fn main(){
    // movies::play("Herold and Kumar".to_string());
    play("Herold and Kumar".to_string());
    play1("danaotiangong".to_string());

}

//使用mod来定义模块，如果想公开使用pub关键字
pub mod movies {
    pub fn play(name:String) {
        println!("Playing movie {}", name);
    }
}

//多层嵌套
pub mod movie{
    pub mod english {
        pub mod comedy {
            pub fn play1(name:String){
                println!("movie is {}", name);
            }
        }
    }
}
