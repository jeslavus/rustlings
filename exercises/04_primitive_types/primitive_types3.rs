fn main() {
    let mut a = [0; 100];

    for i in 0..100 {
        a[i] = i as i32; 
    }

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
