/*
homework requirement:
student ID: 1252
submit address: https://wj.qq.com/s2/11605277/042e

1. 为枚举交通信号灯实现一个 trait，trait里包含一个返回时间的方法，不同的灯持续的时间不同
请放github链接或直接填写在该问卷文本框里

2. 实现一个函数，为u32类型的整数集合求和，参数类型为 &[u32]，返回类型为Option，溢出时返回None
请放github链接或直接填写在该问卷文本框里

3. 实现一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数，比如圆形，三角形，正方形，需要用到泛型和泛型约束
*/

// rust playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021

// enum IpAddr {
//     V4(String),
//     V6(String),
// }
// let home = IpAddr::V6(String::from("127.0.0.1"));

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



