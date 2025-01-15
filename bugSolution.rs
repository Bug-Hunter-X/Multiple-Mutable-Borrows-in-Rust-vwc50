fn main() {
    let mut x = 5;
    { //Scope for the mutable borrow
        let y = &mut x; 
        *y += 1;
    }
    { //Scope for the other mutable borrow
        let z = &mut x; 
        *z += 1; 
    }    
    println!("{}", x);
}