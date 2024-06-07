//It is a concept in rust to understood properly
//There are two types of memory 
//Stack and Heap
//Stack is the faster memory than Heap
//Ownership purpose -it manage the data in heap 

//Rules
//Every variables has a owner
//only one owner
//When the owner goes out of the scope the value is dropped

pub fn owner(){ 
    println!("Executed the owner function");
//variable stored in stack
    let a =1;
    println!("The value of the variable:{a}");
//This variable is stored in the heap
  let string=String::from("Name");
     println!("String is {}",string);
//This variable is stored in heap and it is mutable
  let mut string1=String::from("Rust");
    string1.push_str("programming");
    println!("String1 is {}",string1);
   
}//a, string, string1 are not valid from this point

//copy  Trait 
pub fn copy() {
//this exceute the copy trait in rust 
//This happen when value is stored in stack
  let amove=20;
  let bmove=amove;
  println!("Executed the moving function");
//to print the value of amove 
  println!("The  value is amove :{amove}");
//to print the value if bmove
  println!("The value is bmove :{bmove}")
}

//Move trait 
pub fn mov(){ 
   //This move trait is executes when value is stored in the heap
   //The value is stored in heap and the owner is letter
  let letter=String::from("Rust");
  //Here the ownership change to letter1
  let letter1=letter;
  println!("Executed the mov  function");
  //So the value is move to letter1
  println!("The letter is :{letter1}");

  //This can be achieved by clone method
  let letter2=letter1.clone();
  println!("The letter2 is :{letter2}")
}
