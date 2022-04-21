fn main() {
    let s = String::from("Hello wow");
    let word = find_first_word(&s);
    
    println!("{}",word);

    let last = find_last_word(&s);

    println!("{}", last);



    //s.clear(); will error. &str is a string literal, slices are still the same as "let s = 'literal'"
}

fn find_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn find_last_word(s:&str) -> &str {
    let bytes =s.as_bytes();

    for (i,&item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[i+1..s.len()] //slices have two parts, the starting element and the ending element. This grabs characters from a string or other lists of objects. 
        }
    }

    &s[..] // this is the same as [0..s.len()]
}