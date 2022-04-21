fn main() {
    simple_heap_vs_stack_examples();
    interacting_with_variables_on_heap();
    interacting_with_integers_on_stack();
}

fn simple_heap_vs_stack_examples() {
    let mut x = 10; //x is a literal, and allocated to the stack

    let y = x; //since x is on the stack, y can have the same value set to it.

    println!("{},{}",x,y); //since its on the stack x can still be used, because y=x essentially cloned x to y.

    x = 11;

    println!("{},{}",x,y); //further demonstrating that x and y are not the same object. 

    let s1 = String::from("TEST"); //Strings or complex objects are allocated to the heap.
    let s2 = s1; //This means when setting s2 = s1, s2 takes ownership of the memory on the heap. s1 is no longer valid.

    println!("{}",s2);
    //println!("{}",s1); would error since s1 is no longer valid ref.
}

fn interacting_with_variables_on_heap(){
    let as1 = String::from("ATEST");
    let as2 = as1;

    println!("{}",as2); //again, in this situation, as1 is no longer a valid ref to the heap. as2 is now the owner for that heap data.

    let bs1 = String::from("BTEST");
    let bs2 = bs1.clone(); // This is a more expensive action, because it 'clones' the heap data and makes bs2 the reference to that cloned data.

    println!("{},{}",bs1,bs2);
}

fn interacting_with_integers_on_stack(){

    let x = 10;
    let y = x;

    println!("{},{}",x,y); // This still works because Rust has a 'copy' trait for objects stored entirely on the stack. TLDR: if an object implements 'copy' trait you can y=x and still use x.

    //variable types that support the copy trait are
    //integer types, boolean,float,char,tuples(if implemented),

}
