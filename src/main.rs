use std::io;
use std::thread;
use std::time::Duration;

static mut TODO: Vec<String> = Vec::new();

static PRIMARY_HEADER: &str = r#"
 ______  _____   ____    _____   __  __  ______  ______  _____   ____
/\__  _\/\  __`\/\  _`\ /\  __`\/\ \/\ \/\  _  \/\__  _\/\  __`\/\  _`\
\/_/\ \/\ \ \/\ \ \ \/\ \ \ \/\ \ \ `\\ \ \ \L\ \/_/\ \/\ \ \/\ \ \ \L\ \
   \ \ \ \ \ \ \ \ \ \ \ \ \ \ \ \ \ , ` \ \  __ \ \ \ \ \ \ \ \ \ \ ,  /
    \ \ \ \ \ \_\ \ \ \_\ \ \ \_\ \ \ \`\ \ \ \/\ \ \ \ \ \ \ \_\ \ \ \\ \
     \ \_\ \ \_____\ \____/\ \_____\ \_\ \_\ \_\ \_\ \ \_\ \ \_____\ \_\ \_\
      \/_/  \/_____/\/___/  \/_____/\/_/\/_/\/_/\/_/  \/_/  \/_____/\/_/\/ /
"#;

static ADD_HEADER: &str = r#"
 ______  ____    ____      
/\  _  \/\  _`\ /\  _`\    
\ \ \L\ \ \ \/\ \ \ \/\ \  
 \ \  __ \ \ \ \ \ \ \ \ \ 
  \ \ \/\ \ \ \_\ \ \ \_\ \
   \ \_\ \_\ \____/\ \____/
    \/_/\/_/\/___/  \/___/ 
"#;

static DELETE_HEADER: &str = r#"
 ____    ____    __       ____    ______  ____
/\  _`\ /\  _`\ /\ \     /\  _`\ /\__  _\/\  _`\
\ \ \/\ \ \ \L\_\ \ \    \ \ \L\_\/_/\ \/\ \ \L\_\
 \ \ \ \ \ \  _\L\ \ \  __\ \  _\L  \ \ \ \ \  _\L
  \ \ \_\ \ \ \L\ \ \ \L\ \\ \ \L\ \ \ \ \ \ \ \L\ \
   \ \____/\ \____/\ \____/ \ \____/  \ \_\ \ \____/
    \/___/  \/___/  \/___/   \/___/    \/_/  \/___/
"#;

static EDIT_HEADER: &str = r#"
 ____    ____    ______  ______
/\  _`\ /\  _`\ /\__  _\/\__  _\
\ \ \L\_\ \ \/\ \/_/\ \/\/_/\ \/
 \ \  _\L\ \ \ \ \ \ \ \   \ \ \
  \ \ \L\ \ \ \_\ \ \_\ \__ \ \ \
   \ \____/\ \____/ /\_____\ \ \_\
    \/___/  \/___/  \/_____/  \/_/
"#;


fn main() {
    loop {
        clear();
        primary_screen();
    }
}

fn primary_screen(){
    println!("{PRIMARY_HEADER}");
    get_todos();
    let mut user_choice = String::new();
    println!("Add(A) Delete(D) Edit(E)");
    io::stdin()
        .read_line(&mut user_choice)
        .unwrap();
    match user_choice.trim().is_empty(){
        true => {
            clear();
            println!("Please Enter Valid Input");
            thread::sleep(Duration::from_secs(1));
        }
        false => {
            match user_choice.to_lowercase().trim(){
                "a" => add_screen(),
                "d" => delete_screen(),
                "e" => edit_screen(),
                _=> ()
            }
        }
    }

}

fn clear(){
    print!("\x1B[2J\x1B[1;1H");
}

fn get_todos(){
    let mut number_var = 0;
    unsafe{
        match !TODO.is_empty(){
            true => {
                for todos in TODO.iter(){
                    number_var = number_var + 1;
                    println!("{number_var}. {todos}");
                }
            }
            false => {}
        }
    }
}

fn edit_screen(){
    clear();
    println!("{EDIT_HEADER}");
    get_todos();
    let mut edit_index = String::new();
    let mut user_edit = String::new();
    println!("Enter the Index Of The Todo You Wish To Edit:");
    io::stdin()
        .read_line(&mut edit_index)
        .unwrap();
    clear();
    println!("Enter Your Edit:");
    io::stdin()
        .read_line(&mut user_edit)
        .unwrap();

    match !user_edit.trim().is_empty() {
        true => {
            match edit_index.trim().parse::<usize>(){
                Ok(value) => {
                    unsafe{
                        TODO[value-1] = user_edit;
                    }
                },
                Err(e) => println!("{e}")
            }
            clear();
            println!("Done!");
            thread::sleep(Duration::from_secs(1));
        }
        false => {
            clear();
            println!("Please Enter Valid Input");
            thread::sleep(Duration::from_secs(1));
        }
    }
}

fn delete_screen(){
    clear();
    println!("{DELETE_HEADER}");
    get_todos();
    let mut delete_index = String::new();
    println!("Enter The Index Of The Todo You Wish To Delete (leave blank to return to home screen):");
    io::stdin()
        .read_line(&mut delete_index)
        .expect("error reading user input");
    match delete_index.trim().parse::<usize>(){
        Ok(value) => {
            unsafe{
                match value <= TODO.len(){
                    true => {
                        clear();
                        TODO.remove(value - 1);
                        println!("DONE !");
                        thread::sleep(Duration::from_secs(1));
                    }
                    false => {
                        clear();
                        println!("Enter a valid Index");
                        thread::sleep(Duration::from_secs(1));
                    }
                }
            }
        }
        Err(e) => {
            clear();
            println!("Please Enter Valid Input{e}");
            thread::sleep(Duration::from_secs(1));
        }
    }
}

fn add_screen(){
    clear();
    println!("{ADD_HEADER}");
    get_todos();
    let mut new_todo = String::new();
    println!("Enter Your New Todo:");
    io::stdin()
        .read_line(&mut new_todo)
        .expect("error reading line");
    //let new_todo: &str = new_todo.trim();
    match new_todo.trim().is_empty(){
        true => {
            clear();
            println!("Please Enter A New Todo");
            thread::sleep(Duration::from_secs(1));
        }
        false => {
            clear();
            let new_todo: String = new_todo;
            unsafe{
                TODO.push(new_todo);
            }
            println!("DONE !");
            thread::sleep(Duration::from_secs(1));
        }
    }

}
