//use rand::Rng;
//use rand::thread_rng;

fn generate_random_array() {
//    let mut arr = [0u8;10];
//    thread_rng().gen_range(0u8 .. 10).fill(&mut arr[..]);
//    println!("Random number array {:?}", arr);
}


fn main() {
    let mut a = [3,2,3,4,5];

    //let mut i = 0;
    println!("Starting array: {:?}",a);

    for j in 0..(a.len()-1) {
        let mut count = 0;
        for i in 0..(a.len()-1-j) {
        if a[i] > a[i+1] {
        let temp = a[i];
        a[i] = a[i+1];
        a[i+1] = temp;
        println!("greater {:?}",a);
        } else {
        println!("lesser");
            count = count + 1;
        }
    }
        println!("Length: {}", (a.len()-j-1));
        println!("Count: {}", count);
        if count == (a.len()-j-1) {
            println!("Finished early!");
            break;
        }
    }



    println!("Final array {:?}", a);
    generate_random_array()
}
