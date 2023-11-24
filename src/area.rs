
pub mod area{
    pub fn area_rect<T:std::ops::Mul<Output=T>>(x:T,y:T)->T{
        x*y
    }
    pub fn area_square<T:std::ops::Mul<Output=T> + Copy>(a:T)->T{
        a*a
    }
}