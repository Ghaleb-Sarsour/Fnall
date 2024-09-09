//Importing crates
use std::env;
use walkdir::WalkDir;

    
fn main() {
    let args: Vec<String> = env::args().collect(); //Getting User Input via CLI argument

    //Grabbing the user's home directory to be used as the default search path
    let binding = dirs::home_dir().unwrap();
    let hd = binding.to_str().unwrap();
    let find_file;
    let mut path = String::from(hd.to_string());

    //Checking if the user has entered the correct number of arguments
    if args.len() < 2{
       eprintln!("Usage: fnall FILE [DIRECTORY]"); 
     }
       
    //Checking if the user has entered a directory to search in and using that as the search path
    else if args.len() == 3{
        path = args[2].clone();
    }

    //File that the user wants to find
    find_file = &args[1];

    //Traversing the directory to find the file
    for entry in WalkDir::new(path.clone()).into_iter().filter_map(|e| e.ok()) {
        
        let filename = entry.path().file_name().unwrap(); //Getting the file name from the path
        
        //Checking if the file name matches the file that the user is looking for and printing the path if it does
        if filename.to_str().unwrap() == find_file{
            println!("{}", entry.path().display());
        }

    }
}
