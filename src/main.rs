use rand::Rng;
use std::io;
//use rand::thread_rng;



fn bubble_sort(arr: &mut [i32]) {
    for j in 0..(arr.len()-1) {
        let mut count = 0;
        for i in 0..(arr.len()-1-j) {
        if arr[i] > arr[i+1] {
        let temp = arr[i];
        arr[i] = arr[i+1];
        arr[i+1] = temp;
        //println!("greater {:?}",arr);
        } else {
        //println!("lesser");
            count = count + 1;
        }
    }
        //println!("Length: {}", (arr.len()-j-1));
        //println!("Count: {}", count);
        if count == (arr.len()-j-1) {
            //println!("Finished early!");
            break;
        }
        println!("Array after {} step: {:?}", j+1, arr);
    }
}

fn generate_array(n: usize) -> Vec<i32> {
    println!("{}", n);
    let mut b = vec![0; n];
    for i in 0..n {
        b[i]=rand::thread_rng().gen_range(1..10);
    }
    b
}

fn main() {

    println!("State array size: ");
    let mut size = String::new();
    io::stdin()
        .read_line(&mut size)
        .expect("Failed to read line");
    let size: usize = size.trim().parse().expect("Please type a number!");
    println!("Chosen array size: {}", size);

    let mut a = generate_array(size);

    println!("Starting array: {:?}", a);
    bubble_sort(&mut a);
    println!("Final array {:?}", a);
}
