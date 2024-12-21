use std::io;

struct TodoItem {
    name: String,
    completed: bool,
}
fn main(){

    let mut todo: Vec<TodoItem>=Vec::new();

   loop {
    println!("What would you like to do?");
    println!("1. Add a to-do item");
    println!("2. Complete a to-do item");
    println!("3. Display to-do items");
    println!("4. Quit");
    let mut choice = String::new();

    io::stdin().read_line(&mut choice).expect("Failed to read line");

    match choice.trim() {
        "1"=>{}
        
    }
   }

   

}