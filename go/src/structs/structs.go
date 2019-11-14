package main

import (
	"fmt"
	"org"
)

type Salaried interface {
	getSalary() int
}


func (s Salary) getSalary() int {
	return s.basic + s.insurance + s.allowance
}

type Salary struct{
	basic 		int
	insurance	int
	allowance	int
}

type Employee struct{
	firstName, lastName string
	salary 				int
	fullTime 			bool
	salary_struct 		Salary

}

type Employee_an struct{
	firstName, lastName string
	salary 				int
	fullTime 			bool
	Salary

}

type Employee_inter struct {
	firstName, lastName	string
	Salaried
}


func main() {
	ross := Employee{
		firstName : "ross",
		lastName : "Bing",
		salary : 1200,
		fullTime : true,
	}

	fmt.Println(ross)
	fmt.Println("basic_salary", ross.salary_struct.basic)

	rosa := Employee{"Rosa", "Geller", 1200, true, Salary{1,1,1}}

	fmt.Println(rosa)
	fmt.Println("basic_salary", rosa.salary_struct.basic)

	juan := &Employee{
		firstName : "juan",
		lastName : "perez",
		salary : 2000,
		fullTime : true,
	}

	fmt.Println("firstname", (*juan).firstName)
	fmt.Println("salary", juan.salary)


	pedro := Employee_an{
		firstName: "Pedro",
		lastName: "lopez",
		Salary: Salary{1100,50,50},
	}
	fmt.Println("pedro", pedro)
	pedro.basic = 1200
	pedro.insurance = 0
	pedro.allowance = 0
	fmt.Println("pedro", pedro)

	lalo := Employee_inter{
		firstName: "Pedro",
		lastName: "lopez",
		Salaried: Salary{1100,50,50},	
	}

	fmt.Println("lalo salary ", lalo.getSalary())

	paco := org.Employee_export{
		FirstName: "Paco",
		LastName: "Ramirez",
		//salary: 200,
	}
	fmt.Println(paco)

	
}
