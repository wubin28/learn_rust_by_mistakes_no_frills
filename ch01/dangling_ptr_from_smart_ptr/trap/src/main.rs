fn main() {
    // Declare a variable, but don't initialize it yet
    let raw_ptr;

    { // Start a new scope
        let smart_ptr = Box::new(55);
        // Get a raw pointer from the smart pointer
        raw_ptr = &*smart_ptr as *const i32;

        println!("Value managed by smart pointer: {}", smart_ptr);
        unsafe {
            println!("Value pointed to by raw pointer: {}", *raw_ptr);
        }
    } // smart_ptr is destroyed at the end of this scope, the memory it points to is freed

    // Attempt to use the dangling pointer raw_ptr
    unsafe {
        // Compiles successfully, produces undefined behavior at runtime
        println!("Attempting to access value pointed to by dangling raw pointer (different each run): {}", *raw_ptr);
    }
}
// Runtime output:
// Value managed by smart pointer: 55
// Value pointed to by raw pointer: 55
// Attempting to access value pointed to by dangling raw pointer (different each run): -693338112