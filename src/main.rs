use std::fmt::Debug;

//冒泡排序（Bubble Sort）
fn main() {
    let mut v = [1,2];
    v.swap(v[0],&v[0] + 1);


    println!("Sort numbers ascending");
    let mut numbers = vec![4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    // println!("Before: {:?}", numbers);
    //bubble_sort(&mut numbers);
    // println!("After:  {:?}\n", numbers);

    // println!("Sort strings alphabetically");
    // let mut strings = ["beach", "hotel", "airplane", "car", "house", "art"];
    // println!("Before: {:?}", strings);
    // bubble_sort(&mut strings);
    // println!("After:  {:?}\n", strings);
}


pub fn bubble_sort<T:Ord>(arr: &mut [T]){
    for i in 0..arr.len() {
        println!("iii{}",i);
        for j in 0..arr.len() -1 -i {
            println!("jjj{}",j);
            //panic!("p{:#?}",arr[j]);
            // if(arr[j] > arr[j + 1]){
                arr.swap(j,j + 1);
            // }
        }
    }
}