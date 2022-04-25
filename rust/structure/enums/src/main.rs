fn main() {

    let basic_x = BasicEnums::X; //creating an instance of a enum 
    basic_x.enums_can_have_methods_like_structs(); //calling a method on the enum

    let advanced_no_value= AdvancedEnums::no_value; // again
    advanced_no_value.enums_can_have_methods_like_structs();

    let advanced_single_value = AdvancedEnums::single_value(String::from("wow"));//creating a enum with a value.
    advanced_single_value.enums_can_have_methods_like_structs();

}

enum BasicEnums {
    X,
    Y,
}

enum AdvancedEnums {
    no_value,
    single_value(String),
    mixed_values(u8,u8),
    named_fields{Name: String, Data: u8},

}
impl BasicEnums {
    fn enums_can_have_methods_like_structs(&self){
        println!("Basic enums");
    }
}

impl AdvancedEnums {
    fn enums_can_have_methods_like_structs(&self){
        println!("Advanced enums");

    }
}