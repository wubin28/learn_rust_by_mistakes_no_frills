fn main() {
    let raw_ptr;
    let smart_ptr;

    {
        smart_ptr = Box::new(55);
        raw_ptr = &*smart_ptr as *const i32;

        println!("Value managed by smart pointer: {}", smart_ptr);
        unsafe {
            println!("Value pointed to by raw pointer: {}", *raw_ptr);
        }
    }

    unsafe {
        println!(
            "Attempting to access value pointed to by dangling raw pointer: {}",
            *raw_ptr
        );
    }
}
// Runtime output:
// Value managed by smart pointer: 55
// Value pointed to by raw pointer: 55
// Attempting to access value pointed to by dangling raw pointer: 55