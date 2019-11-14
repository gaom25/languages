package main

import (
	"fmt"
)

func main() {
  ages := make(map[string]int)

  age := map[string]int {
    "mina": 23,
    "john": 25,
    "bob": 30,
  }

  for key, value := range age {
    ages[key] = value
  }

  delete(ages, "john")
  fmt.Println("age", age)
  fmt.Println("ages", ages)
	
}