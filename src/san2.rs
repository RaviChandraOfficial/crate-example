pub fn copy(){
    println!("San2 is accessing");
}
pub mod basics{
    pub mod mult{
        pub fn multi(x:i32,y:i32)->i32{
            x*y
        }
    }
    pub mod add{
        pub fn addi(x:i32,y:i32)->i32{
            x+y
        }
    }
    pub mod sub{
        pub fn subi(x:i32,y:i32)->i32{
            // let mut result=0;
            if x>y{
                // result = x-y;
                x-y
            }
            else{
                // result=y-x;
                y-x
            }
        }
    }
    pub mod div{
        pub fn divi<T:std::ops::Div<Output = T> >(x:T,y:T)->T{
            x/y
        }
    }
}