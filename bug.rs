fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x;  // z is an immutable reference to x

    *y += 1; // Modifying x through y is allowed
    println!("x = {}", x); // Output: x = 6

    // The following line will cause a compile-time error because z is an immutable reference and cannot be used to modify x
    //*z += 1; // Error: cannot assign to immutable borrowed content
}