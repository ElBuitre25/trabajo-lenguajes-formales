use std::io;

fn main() {
    let mut my_input = String::new();

    // Lee cadenas hasta recibir una no vacia y formada solo por (a,b).
    let input_format1 = loop {
        my_input.clear();
        println!("Ingresa la cadena, debe pertenecer al alfabeto (a,b)");

        io::stdin().read_line(&mut my_input).expect("Error");

        let candidate = my_input.trim();

        let input_validate = !candidate.is_empty() && candidate.chars().all(|c| matches!(c, 'a' | 'b'));
        if !input_validate {
            println!("La cadena no pertenece al lenguaje");
        } else {
            break candidate.to_string();
        }
    };

    // 0:0137, 1:247, 2:7, 3:8, 4:68, 5:58, 6:TRAMPA
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

        if estado_actual == 6 {
            return 6;
        }

        // Transicion no definida => estado trampa (rechazo).
        return 6;
    }

    let mut estado_actual: usize = 0; // inicia en 0137

    for caracter in input_format1.chars() {
        estado_actual = transicion(estado_actual, caracter);
    }

    let estado_final = estado_actual;
    
    if estado_final == 1 || estado_final == 3 || estado_final == 4 || estado_final == 5 {
        println!("La cadena es aceptada");
    } else {
        println!("La cadena no es aceptada");
    }
}
