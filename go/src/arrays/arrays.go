package main

import (
	"fmt"
)

func main() {
  a := [...]int{1,2,3,4}
  b := [4]int{1,2,3,4}

  for index, value := range a{
    fmt.Printf("index '%d' - value '%d'\n", index, value)
  }

  for index := 0; index < len(b); index++ {
    fmt.Printf("b[%d] = %d\n", index, b[index]) 
  }

  fmt.Println("slicess")

  s1 := make([]int, 2, 4)
  s2 := []int{1, 2, 3}

  fmt.Printf("s1=> %v, s2=> %v\n", s1, s2)

  s1 = append(s1, 4, 5, 6)
  fmt.Printf("s1=> %v\n", s1)

  s3 := b[1:3]
  fmt.Printf("s3=> %v\n", s3)
  s3 = append(s3, 33)
  fmt.Printf("s3=> %v\n", s3)
  fmt.Printf("b=> %v\n", b)
	
}


//cuando se pasa un array a una funcion esta pasa por valor no por referencia

// los slice crean un nuevo array cuando cambia su tamaño

//type slice struct {
//    zerothElement *type
//    len int
//    cap int
//}
