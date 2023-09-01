# substrate_rust

## Q1
![运行结果](https://github.com/Ashley1024/substrate_rust/blob/main/traffic_light.png)
### enum_learn.rs
```
#[test]
fn trait_main(){
    let red_light: TrafficLight = TrafficLight::Red;
    let yellow_light: TrafficLight = TrafficLight::Yellow;
    let green_light: TrafficLight = TrafficLight::Green;

    println!("red light is {}",red_light.time());
    println!("yellow light is {}",yellow_light.time());
    println!("green light is {}",green_light.time());
}

enum TrafficLight {
    Red,
    Green,
    Yellow,
}

impl TrafficLight {
    fn time(&self) -> u8{
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Green => 30,
            TrafficLight::Yellow => 5,
        }
    }  
}
```
## Q2 
![运行结果](https://github.com/Ashley1024/substrate_rust/blob/main/sum_array.png)
### sum_arr.rs

```
// 2. 实现一个函数，为u32类型的整数集合求和，参数类型为 &[u32]，返回类型为Option，溢出时返回None
// 请放github链接或直接填写在该问卷文本框里

#[test]
fn sum_main(){
    let mut list: Vec<u32> = vec![1,2,3,4,5];
    // list[0];
    // println!("list[0]: {}",list[0]);
    sum_array::<u32>(&mut list);
}

fn sum_array<T: PartialOrd + Copy + std::fmt::Debug + std::fmt::Display>(list: &mut Vec<u32>) -> Option<u32> {
    let mut sum_num = 0;

    let n = list.len();
    if n > 0 {
        for i in 0 ..n{
            sum_num += list[i];
        }

        let opt = Some(sum_num);

        match opt {
            Some(ref x) => println!("input array {:?}, sum result:{}",list, x),
            None => println!("None"),
        }
        return Some(sum_num);
    } else {
        return None;
    }
}

```
## Q3
![运行结果](https://github.com/Ashley1024/substrate_rust/blob/main/calculate_area.png)
### calculate_area.rs
```
// 3. 实现一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数，比如圆形，三角形，正方形，需要用到泛型和泛型约束
// use crate::calculate_obj::shape::Area;
mod shape;
#[test]
fn calculat_main(){

    let radius =6.0;
    let width =10.0;
    let height =9.0;
    let mut cal_circle = crate::calculat_area::shape::shape::Circle::new();
    cal_circle.set_radius(radius);
    let cal_circle = crate::calculat_area::shape::shape::Area::area_format(&cal_circle); 

    println!("If radius is {}, circle area: {}", radius,cal_circle);
 
    let mut cal_rectangle = crate::calculat_area::shape::shape::Rectangle::new();
    cal_rectangle.set_w_h(10.0, 10.0);
    println!("If height is {} and width is {}, rectangle area: {}",height,width, crate::calculat_area::shape::shape::Area::area_format(&cal_rectangle));

    // let mut cal_triangle
    let mut cal_triangle: shape::shape::Triangle = crate::calculat_area::shape::shape::Triangle::new();
    cal_triangle.set_w_h(10.0, 10.0);
    println!("If height is {} and width is {}, triangle area: {}",height,width, crate::calculat_area::shape::shape::Area::area_format(&cal_triangle));


}
```

### calculate_area/shape.rs
```
pub mod shape {
    pub trait Area {
        fn area_format(&self) -> f32;
    }
    #[derive(Debug)]
    pub struct Rectangle {
        width: f32,
        height: f32,
    }
    impl Rectangle {
        pub fn set_w_h(self: &mut Rectangle, w: f32, h: f32) {
            self.width = w;
            self.height = h;
        }
 
        pub fn new() -> Rectangle {
            Rectangle {
                width: 0.0,
                height: 0.0,
            }
        }
    }
    impl Area for Rectangle {
        fn area_format(&self) -> f32 {
            self.height * self.width
        }
    }

    #[derive(Debug)]
    pub struct Circle {
        radius: f32,
    }
 
    impl Circle {
        pub fn set_radius(self: &mut Circle, val: f32) {
            self.radius = val;
        }
 
        pub fn new() -> Circle {
            Circle { radius: 0.0 }
        }
    }
 
    impl Area for Circle {
        fn area_format(&self) -> f32 {
            self.radius * self.radius * 3.14
        }
    }

    pub struct Triangle{
        width: f32,
        height: f32,
    }
    impl Triangle {
        pub fn set_w_h(self: &mut Triangle, w: f32, h: f32) {
            self.width = w;
            self.height = h;
        }
 
        pub fn new() -> Triangle {
            Triangle {
                width: 0.0,
                height: 0.0,
            }
        }
    }
    impl Area for Triangle {
        fn area_format(&self) -> f32 {
            self.height * self.width * 0.5
        }
    }
}

```
