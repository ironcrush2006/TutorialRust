//This is repo with helps to enhance your 
//Knowledge in rust 

//These are called modules in rust 
//Which helps us to organize our code
pub mod variables;
pub mod structs;
pub mod ownership;
//main function  

fn main() 
{ 
    //print function 
    ownership::owner();
    ownership::mov();
    ownership::copy();
    variables::var();
    structs::m();
   
}  