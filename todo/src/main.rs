use std::io;

struct TodoItem {
    id: u32,
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
        "1"=>{
            let mut name = String::new();
            println!("Name of the to-do item:");
            io::stdin().read_line(&mut name).expect("Failed to read line");
            let id = (todo.len() + 1) as u32;  
            let it = TodoItem{
                id,
                name:name.trim().to_string(),
                completed: false,
            };
            todo.push(it);
            },
        "2"=>{
            let mut id= String::new();
            println!("Enter the id of the to-do item you want to complete:");
            io::stdin().read_line(&mut id).expect("Failed to read line");
            let id: u32 = id.trim().parse().expect("Invalid input");
            if let Some(item)=todo.iter_mut().find(|item| item.id == id){

                item.completed = true;
                println!("To-do item {} completed", id);

            }else{
                println!("No item found with ID {}", id);
            }
        },
        "3"=>{
            for i in  &todo{
                println!("{} - {} - {}", i.id, i.name, i.completed);
            }
        }, 

        "4"=>
            break,
        _=> println!("Invalid input"),
        
        }
        
    }
   }

   

