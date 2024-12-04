fn main() {
    let mut x = 5;
    let y = &mut x; // Mutable reference
    let z = &x;  // Immutable reference

    *y += 1; // Modify through mutable reference
    println!("x (modified via mutable reference) = {}", x);

    println!("x (accessed via immutable reference) = {}", z); // Access via immutable reference
    
    // Correct way to modify if needing both mutable and immutable references
    let z_after_mutable = &x; 
    println!("x (accessed via immutable reference after modification) = {}", z_after_mutable);
}