//烧饼的排序
//Pancake Sort
pub fn pancake_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len < 2 {
        return;
    }
    for i in (0..len).rev() {
        // find index of the maximum element within `arr[0..i]` (inclusive)
        let max_index = arr
            .iter()
            .take(i + 1)
            .enumerate()
            .max_by_key(|&(_, elem)| elem)
            .map(|(idx, _)| idx)
            .unwrap();

        if max_index != i {
            flip(arr, max_index);
            flip(arr, i);
        }
    }
}

// function to flip a section of a mutable collection from 0..num (inclusive)
fn flip<E: PartialOrd>(arr: &mut [E], num: usize) {
    arr[0..num + 1].reverse();
}

fn main() {
    println!("Sort numbers ascending");
    let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    println!("Before: {:?}", numbers);
    pancake_sort(&mut numbers);
    println!("After:  {:?}\n", numbers);

    println!("Sort strings alphabetically");
    let mut strings = ["beach", "hotel", "airplane", "car", "house", "art"];
    println!("Before: {:?}", strings);
    pancake_sort(&mut strings);
    println!("After:  {:?}\n", strings);
}