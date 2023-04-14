# rust_terminal_text_game
Rpg TextGame in Rust


Cosas aprendidas:

Valores por defecto en los errores
Para evitar que tu aplicación se detenga por un error y que continúe ejecutándose, puedes colocar un valor por defecto utilizando unwrap_or().

fn main() {
    println!("Ingrese su edad: ");
    let mut edad: String = String::new();
    std::io::stdin().read_line(&mut edad).unwrap();
    let edad_int: u8 = edad.trim().parse().unwrap_or(18);

    println!("Tienes {} años: ", edad_int);	
}
Si el usuario ingresa una letra en lugar de un número cuando se le solicita su edad, la conversión del tipo de dato fallará, pero el unwrap_or() devolverá un valor establecido por defecto y tu aplicación continuará operando.

Estructuras de datos
Crear estructuras de datos para almacenar, dentro de una misma variable, atributos pertenecientes a una misma cosa. Puedes crear estructuras que tendrán la forma de tus datos de la siguiente manera:

struct Person {
    nombre: String,
    apellido: String,
    edad: i32,
    pais: String,
}

fn main() {
    let persona = Person {
        nombre: "Kevin".to_string(),
        apellido: "Fiorentino".to_string(),
        edad: 27,
        pais: "Argentina".to_string(),
    };

    println!("{}", persona.nombre);
    println!("{}", persona.apellido);
    println!("{}", persona.edad);
    println!("{}", persona.pais);
}
Con la palabra clave struct, declaras las propiedades de tu estructura y puedes crear una variable que almacene estos datos y acceder a ellos mediante un punto “.”.

NOTA: Las estructuras utilizan CamelCase para nombrar a las mismas a diferencia de las variables o funciones que utilizan snack_case.

 

Implementaciones de estructuras
Una estructura puede extenderse e implementar funciones para realizar determinada lógica, como crear un nuevo objeto de ese tipo y realizar algún cálculo.

struct Person {
    nombre: String,
    apellido: String,
    edad: i32,
    pais: String,
}

impl Person {
    fn new_person(nombre: String, apellido: String, edad: i32, pais: String) -> Person {
        return Person { nombre, apellido, edad, pais };
    }
}

fn main() {
    let persona: Person = Person::new_person("Kevin".to_string(), "Fiorentino".to_string(), 27, "Argentina".to_string());

    println!("{}", persona.nombre);
}
Utilizando la palabra reservada impl, la función new_person creará un objeto Person pasándole como parámetro los datos que necesita y devolviendo el mismo para su posterior utilización. Has el llamado a estas funciones implementadas en una estructura con Person::XXXXX.

Almacenamiento clave/valor
Otra manera de almacenar información, además de las estructuras y los vectores, son los denominados HashMap. Los mismos son “diccionarios” de datos, del tipo clave/valor, donde para acceder a un elemento, en lugar de utilizar el índice del mismo como en un vector, se utiliza la Clave, que puede ser un string o un número, para colocarle un nombre al Valor y obtenerlo.

use std::collections::{HashMap};

fn main() {
    let mut diccionario: HashMap<&str, &str> = HashMap::new();

    diccionario.insert("Manzana", "La manzana es roja.");
    diccionario.insert("Banana", "La banana es amarilla.");
    diccionario.insert("Naranja", "La naranja es... naranja.");

    println!("{}", diccionario["Manzana"]);			// La manzana es roja.
}
De esta forma, puedes guardar un nuevo valor con insert() y acceder al mismo a través de su clave.

Explora estas nuevas estructuras y tipos de datos para resolver diversas situaciones donde se vuelve algo más complicado manipular la información y mantener la claridad y prolijidad en tu código.

Rust es un lenguaje con una curva de dificultad importante cuando se trata de manipular tipos de datos. Puede ser un dolor de cabeza, así que veamos algunos consejos extras.

Accediendo al índice de un vector
Cuando recorras un array con un for, a veces es interesante saber en qué índice de iteración estamos. Con un simple modificación, puedes obtener este dato en cada iteración.

fn main() {
    let mut my_arr: Vec<String> = Vec::new();

    my_arr.push("Primer valor".to_string());
    my_arr.push("Segundo valor".to_string());
    my_arr.push("Tercer valor".to_string());

    for (index, word) in my_arr.iter().enumerate() {
        println!("{} {}", index, word);
    }
}
Con .iter().enumerate() obtienes el index como primer valor de un ciclo for, y el valor en cuestión en el segundo parámetro. Así, ya puedes aplicar una lógica determinada si el índice es par o impar, por ejemplo.

Option, Some y None
Estas tres palabras reservadas se utilizan muchísimo en Rust y tienes que comprender su funcionamiento para sacarle el máximo provecho y entender qué está sucediendo en tu código.

Recordemos por un momento que Option<T> puede devolver un valor determinado como Some, o puede devolver un tipo de dato None, pero que este es distinto de null. En Rust, el valor nulo simplemente no existe.

fn dividir_numeros(numerador: i128, denominador: i128) -> Option<i128> {
    if denominador == 0 {
        None
    } else {
        Some(numerador / denominador)
    }
}

fn main() {
    if let Some(result) = dividir_numeros(10, 2) {
        println!("El resultado es: {}", result)
    } else {
        println!("No puedes dividir por cero");
    }
}
En este ejemplo, la función dividir_numeros() evaluará el denominador y, si este es cero, retornara un None, de lo contrario, devolverá el resultado de la división y puedes validar este directamente en el if let Some(result) ... para crear la variable si la función devolvió un Some y luego mostrar la misma.

Otra forma de validar un Option ya sea por Some o None es con la palabra reservada match:

fn dividir_numeros(numerador: i128, denominador: i128) -> Option<i128> {
    if denominador == 0 {
        None
    } else {
        Some(numerador / denominador)
    }
}

fn main() {
    let result = dividir_numeros(10, 2);

    match result {
        Some(value) => { println!("El resultado es: {}", value) },
        None => { println!("No puedes dividir por cero"); },
    }
}
match evaluará el resultado de la función por Some o por None, ejecutando el código según corresponda.

Estos conceptos realmente requieren de tiempo y paciencia para comprender. Si vienes de lenguajes como C++, tal vez te sientas algo familiarizado con la utilización de los caracteres & o de un *, pasajes por valor o referencia. Apóyate siempre de la documentación oficial de Rust para comprender cómo se utiliza cada característica y por qué.