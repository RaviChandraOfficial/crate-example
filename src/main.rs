use array;
use array::san;
use array::ravi;
use array::san2;
use array::area;
fn main() {
    array::printing();
    san::sanprint();
    ravi::ravprint();
    println!("{:?}",array::santosh());
    let a=[5,3,8,1,6];
    let c=a.len();
    println!("{:?}",c);
    let  mut b:Vec<i32>=Vec::new();
    for _i in 0..c{
        b.push(a[c-1-_i as usize]);
    }
    println!("reversed array: {:?}",b);
    san2::copy();
    
    println!("Addition of two numbers is {:?}",san2::basics::add::addi(1,2));
    println!("Addition of two numbers is {:?}",san2::basics::div::divi(3, 4));
    println!("Addition of two numbers is {:?}",san2::basics::mult::multi(3, 4));
    println!("Addition of two numbers is {:?}",san2::basics::sub::subi(4, 3));
    let n= area::area::area_rect(3,5);
    let m=area::area::area_square(5.0);
    println!("Area of the rectangleis: {:?}",n);
    println!("Area of the square is: {:?}",m);
}
