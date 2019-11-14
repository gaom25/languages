package main

import (
	"bufio"
	"errors"
	"fmt"
	"os"
	"strings"
	"strconv"
)

func division(x int, y int) (int, error) {
	if y == 0{
		return 0, errors.New("division por 0 :( ")
	}
	return x / y, nil
}

func main() {
	reader := bufio.NewReader(os.Stdin)
	fmt.Println("Simple Division")
	fmt.Println("---------------------")

	fmt.Println("Introduce first number")
	text1, _ := reader.ReadString('\n')
	text1 = strings.Replace(text1, "\n", "", -1)
	num1, _ := strconv.Atoi(text1)

	fmt.Println("Introduce second number")
	text2, _ := reader.ReadString('\n')
	text2 = strings.Replace(text2, "\n", "", -1)
	num2, _ := strconv.Atoi(text2)

	result, err := division(num1, num2)
	if (err != nil){
		fmt.Println("Error feo: ", err)
	}else{
		fmt.Println("Result: ", result)
	}
	
}
