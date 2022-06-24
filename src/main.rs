fn main() {
    let mut s = String::from("hello");
    s.push_str(" world");

    println!("{}", s);

    let s1 = String::from("first");
    let mut s2 = s1;
    // println!("{}", s1); // Invalid code: borrow of moved value
    //
    //
    //

    cal_length(&mut s2);

    let s4 = &s2;

    println!("{}", s4);

    take_ownership(s2);

    // println!("{}", s2); // Error: borrow of moved value

    let x = 4;
    make_copy(x);

    println!("{}", x); // Work well
}

fn take_ownership(str: String) {
    println!("{}", str);
}

fn make_copy(i: i32) {
    print!("{}", i);
}

fn cal_length(str: &mut String) -> usize {
    str.push_str("mut");
    str.len()
}
