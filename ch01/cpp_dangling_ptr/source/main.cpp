// Include the input/output stream library for console output using std::cout
#include <iostream>
// Include the memory management library for smart pointer support like std::shared_ptr
#include <memory>

int main()
{
  // Create a raw pointer, initialized to nullptr, 
  // indicating it doesn't point to any valid memory address
  int* rawPtr = nullptr;

  {  // Start a new scope to limit the lifetime of smartPtr
    // Create smart pointer smartPtr, 
    // pointing to an int object with value 55,
    // std:: indicates that shared_ptr is a smart pointer provided by the C++ standard library
    std::shared_ptr<int> smartPtr = std::make_shared<int>(55);
    rawPtr = smartPtr.get();  // Get the raw pointer managed by the smart pointer

    std::cout << "Value managed by smart pointer: " << *smartPtr << std::endl;
    std::cout << "Value pointed to by raw pointer: " << *rawPtr << std::endl;
  }  // smartPtr is destroyed at the end of this scope, managed memory is released

  // At this point, rawPtr becomes a dangling pointer
  // Undefined behavior, may see different values in different environments, 
  // or may crash
  std::cout << "Attempting to access value of dangling raw pointer: " << *rawPtr
            << std::endl;  

  return 0;
}
// Run output:
// Value managed by smart pointer: 55
// Value pointed to by raw pointer: 55
// Attempting to access value of dangling raw pointer: 0