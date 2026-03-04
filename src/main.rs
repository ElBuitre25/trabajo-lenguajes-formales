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
    
    let estados = ["0137", "247", "7", "8", "68", "58"];

let mut estado_actual: usize = 0; // empieza en 0137


fn transicion(estado_actual: usize, simbolo: char) -> usize {

    if estado_actual == 0 && simbolo == 'a' {
        return 1;
    }

    if estado_actual == 0 && simbolo == 'b' {
        return 3;
    }

    if estado_actual == 1 && simbolo == 'a' {
        return 2;
    }

    if estado_actual == 1 && simbolo == 'b' {
        return 5;
    }

    if estado_actual == 2 && simbolo == 'a' {
        return 2;
    }

    if estado_actual == 2 && simbolo == 'b' {
        return 3;
    }

   
    if estado_actual == 3 && simbolo == 'b' {
        return 3;
    }
 
    if estado_actual == 4 && simbolo == 'b' {
        return 3;

    }
     if estado_actual == 5 && simbolo == 'b' {
        return 4;
        
    }


    return estado_actual;
}


    
}
