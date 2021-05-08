//use rand::Rng;
//use rand::thread_rng;

fn generate_random_array() {
//    let mut arr = [0u8;10];
//    thread_rng().gen_range(0u8 .. 10).fill(&mut arr[..]);
//    println!("Random number array {:?}", arr);
}

fn main() {
    let mut a = [5,2,8,5,9,5];
    //let mut i = 0;
    println!("Starting array: {:?}",a);

    for i in 0..(a.len()-1) {
        if a[i] > a[i+1] {
        let temp = a[i];
        a[i] = a[i+1];
        a[i+1] = temp;
        println!("greater {:?}",a);
        } else {
        println!("lesser");
        }
    }

    println!("Hello, world!");
    generate_random_array()
}
