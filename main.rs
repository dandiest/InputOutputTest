use std::io;           // use standard input output library


fn create_and_clean() -> String {     
    let mut buffer = String::new();                        // Initialize a new, empty String buffer to store the raw input 
    
    io::stdin().read_line(&mut buffer).expect("Error");   // Read a line from standard input; this includes the "newline" character (\n or \r\n)
    
    buffer.trim().to_lowercase()                         // .trim() removes leading/trailing whitespace and the newline character, .to_lowercase() converts the result to lowercase and allocates a new String.
                                     
}
   


fn main(){
    loop{                                       // Start an infinite loop to keep the program running
        println!("Welcome, type something.");
        let command = create_and_clean();      // Call our helper function to get a "clean" (trimmed and lowercase) string
        if command == "exit" {               
            println!("Bye!"); 
            break;                            // break the loop and stop the program
        }
        println!("You typed {}", command) 
    }
}




     