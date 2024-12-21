use std::io;


fn main(){
    let mut x = String::new();
    let mut y = String::new();

    let mut op = String::new();

   loop {
    println!("Enter the first number: ");
    io::stdin().read_line(&mut x).expect("Failed to read line");

    println!("Enter the second number: ");
    io::stdin().read_line(&mut y ).expect("Failed to read line");

    println!("List of operators:");
    println!("(1) Add");
    println!("(2) Subtract");
    println!("(3) Multiply");
    println!("(4) Divide");
    println!("(5) exit");

    println!("Select the number associated with the desired operation: ");

    io::stdin().read_line(&mut op).expect("Failed to read line");

    let x:f64 = match x.trim().parse() {
        Ok(num)=> num,
        Err(_)=>{
            println!("Invalid input. Please enter a number");
            return;
        }
    };

    let y:f64 = match y.trim().parse(){
        Ok(num)=> num,
        Err(_)=>{
            println!("Invalid input. Please enter a number");
            return;
        }
    };

    let result:f64;
     match op.trim() {

        "1"=> result= x+y,
        "2"=> result = x-y,
        "3"=>result = x*y,
        "4"=>{
            if y == 0.0 {
                println!("Cannot divide by zero");
                return;
            }
            result= x/y
        },
        "5"=> return,
        _=>{
            println!("Invalid operator");
            return;
        }
     };

     println!("The result is: {}", result);
   }


}

