// 3. 实现一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数，比如圆形，三角形，正方形，需要用到泛型和泛型约束
use crate::{calculat_area::shape::shape::Area};
mod shape;
#[test]
fn calculat_main(){
    let mut a = crate::calculate_obj::shape::Circle::new();

    a.set_radius(6);
    let a = crate::calculate_obj::shape::Area::calculate_area(&a); 

    println!("circleArea: {}", a);
 
    let mut b = shape::shape::Rectangle::new();
    b.set_w_h(10, 10);
    println!("rectangleArea: {}", b.calculate_area());

}
