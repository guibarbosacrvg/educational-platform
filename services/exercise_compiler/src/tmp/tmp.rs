
// Type your code here
fn bubblesort(array: &mut [i32]) {
    for i in 0..array.len() {
        for j in 0..array.len() - 1 {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let mut array = [10, 5, 2, 3];
    bubblesort(&mut array);
    println!("{:?}", array);
}