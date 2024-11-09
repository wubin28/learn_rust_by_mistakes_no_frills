fn main() {
    let reference;
    let smart_ptr;

    {
        smart_ptr = Box::new(55);
        reference = &*smart_ptr;

        println!("Value managed by smart pointer: {}", smart_ptr);
        println!("Value pointed to by reference: {}", reference);
    }

    println!("Value pointed to by reference: {}", reference);
}
// Runtime output:
// Value managed by smart pointer: 55
// Value pointed to by reference: 55
// Value pointed to by reference: 55
