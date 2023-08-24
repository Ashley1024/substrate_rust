# substrate_rust

### sort.rs

![运行结果](https://github.com/Ashley1024/substrate_rust/blob/main/running_result.png)


```

#[test]
fn sort_main() {
    let arr:[i32;4] = [7,8,30,21];
    let f = bubble_sort(arr);   
    match f {
        Ok(f)=> {
            println!("bubble sort done. {:?}",f);
         },
        Err(e)=> {
            println!("illeagal input: \n{:?}",e);   // 处理错误
         }
    }

    let mut list = vec![1, 50, 200, 34, 2, 100,44];
    template_sort(&mut list);
}


fn bubble_sort(mut arr:[i32;4])->Result<bool,String>{
    println!("input of fixed type array sort: {:?}",arr);   

        let n = arr.len();
        if n > 1 {
            for index in 0..n-1{
                if arr[index] > arr[index+1] {
                    // let tmp = arr[index];
                    // arr[index] = arr[index+1];
                    // arr[index+1] = tmp; 
                    arr.swap(index,index+1);
                }
            }
            println!("fixed type sort result: {:?}",arr);   
            return Ok(true);
        } else {
            return Err("please make sure the lengh of array more than 1".to_string());
        }
    }
    
fn template_sort<T: PartialOrd + Copy + std::fmt::Debug>(list: &mut Vec<T>) -> &Vec<T> {
    println!("input of arbitrary type array sort: {:?}",list);   

    for i in 0..list.len()-1 {
        if list[i] > list[i+1] {
            list.swap(i,i+1);
        }
    }
    println!("arbitrary type sort result: {:?}",list);   

    list
}
    
