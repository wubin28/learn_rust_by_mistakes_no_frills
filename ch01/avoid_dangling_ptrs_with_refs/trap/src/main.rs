fn main() {
    // Declare a variable named reference, but don't initialize it yet
    let reference;

    { // Start a new scope
        // Bind a smart pointer smart_ptr of type Box<i32>, storing the integer value 55, Box allocates memory on the heap
        let smart_ptr = Box::new(55);
        // The type of variable reference will only be inferred when it's initialized here
        // Explanation of the expression &*smart_ptr:
        // First, the * dereference operator dereferences the Box smart pointer to its i32 value managed on the heap
        // Then the & reference creation operator creates a reference to this heap value
        // Finally, this reference is initialized to the variable reference
        reference = &*smart_ptr;

        println!("Value managed by smart pointer: {}", smart_ptr);

        println!("Value pointed to by reference: {}", reference);
    } // smart_ptr is destroyed when this scope ends

    // Attempting to use the dangling reference will result in a compilation error
    println!("Value pointed to by reference: {}", reference);
}
// Compilation error:
// error[E0597]: `*smart_ptr` does not live long enough
