package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	"strconv"
)

type CalcFunc func(int, int) int

func get_numbers() (int, int) {
  reader := bufio.NewReader(os.Stdin)
  fmt.Println("Introduce first number")
  text1, _ := reader.ReadString('\n')
  text1 = strings.Replace(text1, "\n", "", -1)
  num1, _ := strconv.Atoi(text1)

  fmt.Println("Introduce second number")
  text2, _ := reader.ReadString('\n')
  text2 = strings.Replace(text2, "\n", "", -1)
  num2, _ := strconv.Atoi(text2)

  return num1, num2

}

func exit_message() {
  fmt.Println("Bye Bye now yes")
}

func calc(x, y int, f CalcFunc) int {
  return f(x, y)
}

func add(x, y int) int {
	return x + y
}

func add_mul(x, y int) (add, mul int) {
  add = x + y
  mul = x * y
  return
}

func help(){
	menu := `The command to execute are:
		'bai' to exit the program
		'add' to add two numbers
    'add_mul' add and multiply two numbers
    'calc_add' add two number with function as argument
    'anonymous_mul' multiply two number with anonymous function
		'help' to show this menu`
	fmt.Println(menu)
}

func main() {
  defer exit_message()
  reader := bufio.NewReader(os.Stdin)
	fmt.Println("Simple Shell")
	fmt.Println("---------------------")

  	for {
  		fmt.Print("-> ")
    	text, _ := reader.ReadString('\n')
    	// convert CRLF to LF
    	text = strings.Replace(text, "\n", "", -1)

      switch text {
        case "add":
          num1, num2 := get_numbers()
          result := add(num1, num2)
          fmt.Println("Result: ", result)
        case "add_mul":
          num1, num2 := get_numbers()
          res1, res2 := add_mul(num1, num2)
          fmt.Println("Result1: ", res1)
          fmt.Println("Result2: ", res2)
        case "calc_add":
          num1, num2 := get_numbers()
          result := calc(num1, num2, add)
          fmt.Println("Result: ", result)
        case "anonymous_mul":
          num1, num2 := get_numbers()
          result := func(x, y int) int {
            return x * y
          }(num1, num2)
          fmt.Println("Result: ", result)
        case "bai":
          fmt.Println("Bye bai")
          return
        case "help":
          help()
        default:
          fmt.Println("Wrong command")
          help()
      }
  	}
	
}
