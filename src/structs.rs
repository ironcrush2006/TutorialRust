//This the decelaration for the in rust 



pub struct Rectangle{ 

    width:u32,
    height:u32
}
//impl block it acts like a methods in other languages like C++ and Java
impl Rectangle{ 
    //self copy the instance of the struct 
    pub fn area(&self)-> u32 {
        self.width*self.height
    }

    pub fn check(&self,other:&Rectangle)->bool{ 
         &self.width  >  &other.width   || &self.height > &other.height  
    }
}

pub fn m() {
    let rect=Rectangle{ 
        width:32,
        height:50
    };
    let rect1=Rectangle{ 
        width:12,
        height:30
    };
    println!("The area is :{}",rect.area());
    println!("The check is:{}",rect.check(&rect1))
}