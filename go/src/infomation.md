# Go

###Es un lenguaje hecho para google, diseñado por Google para resolver los problemas de Google.


### Go es:
* Go es de la familia de C y muy parecido a a C es por ello que:
	* Sintaxis como la de C y C++, es por eso que es
	* Compilado(necesita un proceso de compilacion antes de ser ejecutado)
	* De tipado estatico
	* Con uso de apuntadores
	* Sin clases, mas bien con tipos
* Entre los cambios que tiene con podemos nombrar:
	* No hay apuntador aritmetico
	* No hay conversion numerica implicita
	* Limites de los arreglos siempre son checkeados
	* No hay alias de tipos (despues de `type X int`, X y int son distintos tipos no alias)
	* ++ y -- son declaraciones no expresiones
	* asignacion no es una expresion
	* es legal tomar la direccion de memoria de una variable en el stack
* Entre las diferencias que tiene con C:
	* Concurrencia(se pueden compartir variables entre hilos ps usa memoria compartida)
	* Recolector de basura(tiene menos apuntadores al alocar memoria, lo cual aumenta la eficiencia del recolector de basura al estar al tanto de menos referencias)
	* tipos como interfaces
* Go fue diseñado con claridad por eso tiene sintaxis clara.
* De tipado de objetos estatico(el tipo de la variable se conoce a tiempo de compilacion)
* Esta orientado para el servicio de la  ingeniera de software
* Go tiene funciones de primer orden y clausuras(Programacion funcional)
* Go no tiene argumentos por defecto en las funciones, esto con la intencion de mejorar la claridad y la lectura.
This was a deliberate simplification. Experience tells us that defaulted arguments make it too easy to patch over API design flaws by adding more arguments, resulting in too many arguments with interactions that are difficult to disentangle or even understand. The lack of default arguments requires more functions or methods to be defined, as one function cannot hold the entire interface, but that leads to a clearer API that is easier to understand. Those functions all need separate names, too, which makes it clear which combinations exist, as well as encouraging more thought about naming, a critical aspect of clarity and readability.
* No tiene clases ni herencia, usa polimorfismo y composicion para ser Orientado a Objectos https://flaviocopes.com/golang-is-go-object-oriented/
	* Composicion sobre herencia
	* duck typing para saber el tipo de las interfaces
* La visibilidad de un identificador(variables, tipos, metodos, constantes, campos) de un paquete depende de como se declare la variable
	* Si la primera letra es mayuscula la variable es publica
	* En otro caso o con _ al inicio la variable es privada
* No hay `this` uno siempre tiene que usar el nombre del paquete importado y lo que vaya a usar, debido a que todas las variables tienen el mismo alcance. Es decir todas las variables son locales asi x.Y se traduce en busca el paquete x localmente ya que la variable Y le pertenece.
* Un tipo no puede tener varias funciones declaradas con el mismo nombre debido que la busqueda de la funcion se hace por nombre no por su firma
* Imports and compilation time are optimized which not have:
	* circular dependency
	* all imports are defined in the begining of the file
	* the is not recompilation there is transitivity:
		* package A imports package B;
		* package B imports package C;
		* package A does not import package C
* diferencia entre el operador = y el := el primero es solo para asignacion el segundo es para asignacion y declaracion eg: 

	`For example, var foo int = 10 is the same as foo := 10`

Unlike C where a pointer can be incremented or decremented, Go does not allow pointer arithmetic. If you tried to do so, Go will throw compilation error.

### Benchmark
https://benchmarksgame-team.pages.debian.net/benchmarksgame/fastest/go-python3.html

### Tutorial
https://medium.com/rungo


### bootcamp
http://www.golangbootcamp.com/book/tricks_and_tips


### Compare
https://stackshare.io/stackups/go-vs-python