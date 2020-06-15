fn main() {
    let _immutale_bindinng = 1;
    let mut mutable_binding = 1;

    println!("Before mutaion:{}", mutable_binding);

    mutable_binding += 1;

    println!("After mutaion:{}", mutable_binding);

    // _immutale_bindinng += 1;
}
