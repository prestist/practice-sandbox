fn main() {
    let s = String::from("TEST");
    println!("{}",s);
    takes_ownership(s);
    //println!("{}",s); this would error after calling the function, as the heap is now assigned to a different variable. 

    let s1 = give_ownership(); //s1 owns the heap data resulting from "give_ownership()"
    println!("{}", s1);

    let temp = String::from("WOW");
    let s2 = take_and_give_ownership(temp);

    println!("{}", s2);// since the function return's ownership s2 owns the data at this point.

    let length = pass_a_ref_of_object(s2);

    println!("{},{}",s2,length); //s2 is still a valid variable as it still owns the data because we only passed the ref using &

}

fn takes_ownership(_incoming_string: String) {
    //once a value is passed into this function, its similar to s2 = s1. as this '_incoming_sting' variable takes ownership of the heap allocation. 
}

fn give_ownership() -> String{
    let s = String::from("ITS YOURS!");

    s //return s to the calling code.
}

fn take_and_give_ownership(s: String) -> String{
    
    s //return the ownership to calling function.
}

fn pass_a_ref_of_object(s: &String) -> usize{
    //Ref's can act on info in the Stack
    s.len()
    //They cannot alter the data i.e. s.push_str() would error.
}