# Rust

### Lenguaje hecho como proyecto personal dentro de Mozilla y que este lo ayudo en su desarrollo Es un lenguaje multiparadigma enfocado en ser sano es especial concurrencia segura

### Ruest es:
* sintacticamente parecido a c++, pero en su uso es mas parecido haskell
* compilado, tipado estatico, concurrente, tipado fuerte, funcional, imperativo, actores concurrentes
* diseñado para alta concurrencia y seguridad en los sistemas
* la ultima expresion en la funcion crea el valor a retornar
* el lenguaje es diseñado para ser seguro en memoria por eso no permite:
	* apuntadores nulos
	* apuntadores colgantes
	* condiciones de carrera en codigos seguros
	* en vez de null tiene Some 
* No tiene recolector de basura en cambio tiene RAII(Resource Acquisition Is Initialization)
* Tiene conceptos de referencia que se verifican a tiempo de compilacion para no dejar apuntadores colgantes
* los valores se pueden pasar por:
	* referencia inmutable
	* referencia mutable
	* valor
* tiene un mecanismo similar a las clases llamadas "traits" inspiradas en haskell
* `impl` osea implementaciones es lo que hace de clases en otro lenguajes
* herencia y polimorfismo se hacen por medio de `traits` 
* structs tienen solo atributos, traits tienen solo metodos y se conectan via impl
* tipos estructurados son usados para definir campos
* implementaciones y traits no pueden definir campos ellos mismos.
* solo traits proveen herencia
* todo esto previene el problema del diamante en herencia multiple
* Rust provee herencia pero reemplaza implementacion hereditaria con composicion es decir `composicion sobre herencia`
* inferencia de tipos para variable declaradas con `let`
* En tiempo de compilacion los errores salen cuando no se asigna un valor a un variable
* metaprogramacion por medio de macros. Es decir codigo que escribe otro codigo

* Ownership is Rust’s most unique feature, and it enables Rust to make memory safety guarantees without needing a garbage collector. Therefore, it’s important to understand how ownership works in Rust. In this chapter, we’ll talk about ownership as well as several related features: borrowing, slices, and how Rust lays data out in memory. All programs have to manage the way they use a computer’s memory while running. Some languages have garbage collection that constantly looks for no longer used memory as the program runs; in other languages, the programmer must explicitly allocate and free the memory. Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks at compile time. None of the ownership features slow down your program while it’s running.
	* Each value in Rust has a variable that’s called its owner.
    * There can only be one owner at a time.
    * When the owner goes out of scope, the value will be dropped.
 Variable bindings have ownership of what they’re bound to. A piece of data can only have one owner at a time. When a binding goes out of scope, Rust will free the bound resources. This is how Rust achieves memory safety.
* cargo para crear los proyectos, ademas de la documentacion, el release, los test etc
* existen combinatorias para combinar 2 valores Result o Option y devolver el resultado combinado entre las funciones tenemos:
	* or, and, or_else, and_else
	* filter (for Option)
	* map, map_err, convierte un tipo T en tipo U aplicando una clausura
	* map_or, map_or_else, transforma el tipo T usando una clausura y retorna el valor dentro del tipo T
	* ok_or, ok_or_else, para Option transforma Option en Result
	* as_ref, as_mut, transforma el tipo T en una referencia o referencia mutable


### Benchmark


### Tutorial 
https://medium.com/learning-rust

### Compare
https://stackshare.io/stackups/python-vs-rust

### Crash Course
https://www.snoyman.com/blog/2018/10/introducing-rust-crash-course