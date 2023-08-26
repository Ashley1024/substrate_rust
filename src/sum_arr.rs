// 2. 实现一个函数，为u32类型的整数集合求和，参数类型为 &[u32]，返回类型为Option，溢出时返回None
// 请放github链接或直接填写在该问卷文本框里

#[test]
fn sum_main(){
    let mut list: Vec<u32> = vec![1,2,3,4,5];
    // list[0];
    // println!("list[0]: {}",list[0]);
    sum_array::<u32>(&mut list);
}

fn sum_array<T: PartialOrd + Copy + std::fmt::Debug>(list: &mut Vec<u32>) -> Option<u32> {
    let mut sum_num = 0;

    let n = list.len();
    if n > 0 {
        for i in 0 ..n-1{
            sum_num = sum_num + list[i];
        }

        let opt = Some(sum_num);

        match opt {
            Some(ref x) => println!("sum result:{}", x),
            None => println!("None"),
        }
        return Some(sum_num);
    } else {
        return None;
    }
}
