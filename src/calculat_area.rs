// 3. 实现一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数，比如圆形，三角形，正方形，需要用到泛型和泛型约束

fn calculat_main(){
    let mut list: Vec<u32> = vec![1,2,3,4,5];
    // list[0];
    // println!("list[0]: {}",list[0]);
    calculat_area::<u32>(&mut list);
}

fn calculat_area<T: PartialOrd + Copy + std::fmt::Debug>(list: &mut Vec<u32>) -> Option<u32> {

}