package main

import (
	"fmt"
)

type Shape interface {
	Area()		float64
	Perimeter()	float64
}

type Object interface {
	Volume() 	float64
}

type Skin interface {
	Color() float64
}
type Material interface {
	Shape
	Object
}

type Rect struct{
	width 		float64
	height 		float64
}


func (r Rect) Area() float64 {
	return r.width * r.height
}

func (r Rect) Perimeter() float64 {
	return 2 * (r.width + r.height)
}

type Cube struct{
	side 	float64
}

func (c Cube) Area() float64 {
	return 6 * (c.side * c.side)
}

func (c Cube) Perimeter() float64 {
	return 12 * c.side
}

func (c Cube) Volume() float64 {
	return c.side * c.side * c.side
}


func main() {
	var s Shape
	fmt.Println("value of s is ", s)
	fmt.Println("type of s is %T\n", s)

	s = Rect{5.0, 4.0}
	r := Rect{5.0, 4.0}
	fmt.Println("value of s is %v\n", s)
	fmt.Println("type of s is %T\n", s)
	fmt.Println("value of s is ", s)
	fmt.Println("area of s ", s.Area())
	fmt.Println("value of s is ", s)
	fmt.Println("s == r is ", s==r)

	c := Cube{4}
	s = c
	var o Object = c

	fmt.Println("Area ", s.Area())
	fmt.Println("Volume ", o.Volume())
	fmt.Println("Perimeter ", s.Perimeter())

	var e Shape = Cube{3}
	f := e.(Cube)
	fmt.Println("Area ", f.Area())
	fmt.Println("Volume ", f.Volume())
	fmt.Println("Perimeter ", f.Perimeter())
	// In type assertion syntax i.(Type), if Type 
	// does not implement the interface (the type of) 
	// i then Go compiler will throw an error. 
	// But if Type implements the interface but i does 
	// not have a concrete value of Type then Go 
	// will panic in runtime

	value1, ok1 := e.(Object)
	fmt.Println("dynamic value of Shape 'e' with value %v implements interface object? %v", value1, ok1)
	value2, ok2 := e.(Skin)
	fmt.Println("dynamic value of Shape 'e' with value %v implements interface object? %v", value2, ok2)
	//value3 := e.(Skin)
	//fmt.Println("panic", value3)

	cube := Cube{4}
	var m Material = cube
	var shape Shape = cube
	var obj Object = cube

	fmt.Printf("type and value for material '%T' and '%v'\n", m, m)
	fmt.Printf("type and value for Shape '%T' and '%v'\n", shape, shape)
	fmt.Printf("type and value for Object '%T' and '%v'\n", obj, obj)






}
