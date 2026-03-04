use std::io;


fn main() {
    
    let mut my_input: String= String::new();

    // Lee los strings por la terminal por la terminal
    loop {
        my_input.clear();
        println!("Ingresa la cadena, debe pertenecer al alfabeto (a,b)");

        io::stdin().read_line(&mut my_input).expect("Error");  

        let input_format1=my_input.trim();// formateamos a texto legible 

        let input_validate=!input_format1.is_empty() && input_format1.chars().all(|c| matches!(c, 'a' | 'b'));
        if !input_validate{
            println!("La cadena no pertenece al lenguaje");
        }else {
            break;
        }
        

          
    }
    



    
}
