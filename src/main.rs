//importing crates
use std::env;
use walkdir::WalkDir; //Directory Traversal //User Input cli argument

    
fn main() {
    let args: Vec<String> = env::args().collect(); //Getting User Input cli argument

    let binding = dirs::home_dir().unwrap();
    let hd = binding.to_str().unwrap();
    let find_file;
    let mut path = String::from(hd.to_string());

    if args.len() < 2{
       eprintln!("Usage: goto 'path' 'scanning directory'*Optional* "); 
     }
       
    else if args.len() == 3{
        path = args[2].clone(); //taking cli arg and putting it into path
    }

    find_file = &args[1];

    for entry in WalkDir::new(path.clone()).into_iter().filter_map(|e| e.ok()) {
        //going though path directory
        let filename = entry.path().file_name().unwrap();
        
        if filename.to_str().unwrap() == find_file{

            println!("{}", entry.path().display());
        }

    }
}
