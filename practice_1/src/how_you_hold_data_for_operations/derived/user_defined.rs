#![allow(dead_code)] //This suppresses warnings when a given declared function is  not used.

use core::cmp::Ordering;

pub enum Comp {
    //Enumerate Comparison
    LessThan,
    GreaterThan,
    Equal,
}

pub enum Gender {
    //Enumerate Gender
    Male,
    Female,
}

#[derive(Debug)] //Decorate the struct Person. Debug is an inbuilt trait. This statement will provoke impl Debug for Person; Metaprogramming
struct Person {
    name: String,
    age: u8,
}
struct Unit;
// A unit struct
//Have no state of their own but useful for
//implementing some trait.
//E.g. struct Global is a unit struct that can implement traits like Allocator
//std::fmt::Error is a unit struct that implements
//traits like Error

//A tuple struct
struct Pair(i32, f32); //No named fields but has fields

//A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct. Below Point is used as datatype in Rectangle
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

pub fn run() {
    //declare a variable of type Person and assign values.
    let person = Person {
        name: String::from("Peter"),
        age: 27,
    };
    println!("{:#?}", person); //{:#?} implies pretty debug print person. :? is for debug print and :#? is for pretty debug print

    // Instantiate a unit struct
    let _unit = Unit; //As simple as that. If Unit implements some trait, then _unit will demand those implementations

    //declare a Point
    let point = Point { x: 10.3, y: 0.4 };

    //Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };
    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a let binding.
    let Point {
        x: left_edge, //left_edge here will be declared. If you use x:f32 only, x will be declared.
        y: top_edge,  //top_edge here will be declared. If you use y:f32 only, y will be declared.
    } = point;
    dbg!(&left_edge, &top_edge);

    let _rectangle = Rectangle {
        //I used _ with rectangle to silence warning knowing that the variable is not used.
        //struct instantiation is an expression too as used below in Point
        top_left: Point {
            x: left_edge, //left_edge is available, thanks to the destructuring above
            y: top_edge,  //top_edge is available, thanks to the destructuring above
        },
        bottom_right,
    };

    //Instantiate a tuple struct
    let pair = Pair(1, 0.1);
    //Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    //Destructure a tuple struct
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
}

//Let's work on user-defined traits. Traits enable us achieve polymorphism.
//We are designing Shape below for the purpose of
//specifying all expected functions and methods in any struct that implements Shape.
// ... (previous code)

// The Shape trait with corrections
trait Shape {
    fn new(length: f32, width: f32, name: &'static str) -> Self;
    fn area(&self) -> f32;
    fn set_length(&mut self, length: f32);
    fn get_length(&self) -> f32;
    fn set_width(&mut self, width: f32);
    fn get_width(&self) -> f32;
    fn set_name(&mut self, name: &'static str);
    fn get_name(&self) -> &str;
    fn perimeter(&self) -> f32;
}

// The Rect struct with corrections
#[derive(Default, Debug, Clone)]
struct Rect {
    length: f32,
    width: f32,
    name: &'static str,
}

impl Rect {
    fn default() -> Self {
        Rect {
            length: 1.0,
            width: 1.0,
            name: "default_name",
        }
    }
}

impl Shape for Rect {
    fn new(length: f32, width: f32, name: &'static str) -> Self {
        Rect {
            length,
            width,
            name,
        }
    }

    fn area(&self) -> f32 {
        self.length * self.width
    }

    fn set_length(&mut self, length: f32) {
        self.length = length;
    }

    fn get_length(&self) -> f32 {
        self.length
    }

    fn set_width(&mut self, width: f32) {
        self.width = width;
    }

    fn get_width(&self) -> f32 {
        self.width
    }

    fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }

    fn get_name(&self) -> &str {
        self.name
    }

    fn perimeter(&self) -> f32 {
        2.0 * (self.length + self.width)
    }
}

//implement Partial Eq
impl PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        self.area() == other.area()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Rect {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.area().partial_cmp(&other.area())
    }
    // Provided methods
    //fn lt(&self, other: &Rhs) -> bool { ... }
    //fn le(&self, other: &Rhs) -> bool { ... }
    //fn gt(&self, other: &Rhs) -> bool { ... }
    //fn ge(&self, other: &Rhs) -> bool { ... }
}

//A conversion implementation into String
//Expects a string slice with length, width, name, separated by commas
impl From<&'static str> for Rect {
    fn from(s: &'static str) -> Rect {
        let mut parts = s.split(',');
        let length = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0.0,
        };
        let width = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0.0,
        };
        let name = match parts.next() {
            Some(val) => val,
            None => "",
        };
        Rect {
            length,
            width,
            name: &name,
        }
    }
}

pub fn run2() {
    let rectangle1 = Rect::default();

    println!("{}", rectangle1.length);
    println!("{}", rectangle1.width);
    println!("{}", rectangle1.name);

    let rectangle2 = Rect::new(1.0, 3.0, "Rectangle2");
    let rectangle3 = Rect::from("4,5,Rectangle3");

    //Compare using PartialOrd
    let result1 = rectangle1.partial_cmp(&rectangle2);
    println!(" result1 = {:?}", result1);

    let result2 = rectangle1.le(&rectangle2);
    println!("result2 = {:?}", result2);

    //Compare using PartialEq
    let result3 = rectangle2.eq(&rectangle3);
    println!("result3 = {:?}", result3);

    let result4 = rectangle2.ne(&rectangle3);
    println!("result4 = {:?}", result4);
}

//Exercise
/*
I need similar implementation for Circle and Triangle
Besides Area, I need Perimeter and comparison on the basis of Perimeter
In your submission, I need a comment against every line of code about what it is mearnt to achieve
 */

/*
!! START OF ASSIGNMENT
 */

// Structure representing a circle with a radius
struct Circle {
    radius: f32,
}

// Structure representing a triangle with side lengths a, b, and c
struct Triangle {
    side_a: f32,
    side_b: f32,
    side_c: f32,
}

// Additional functions for calculating perimeter and area for Circle
impl Shape for Circle {
    fn perimeter(&self) -> f32 {
        2.0 * 3.142 * self.radius
    }

    fn area(&self) -> f32 {
        3.142 * self.radius * self.radius
    }

    fn new(length: f32, width: f32, name: &'static str) -> Self {
        todo!()
    }

    fn set_length(&mut self, length: f32) {
        todo!()
    }

    fn get_length(&self) -> f32 {
        todo!()
    }

    fn set_width(&mut self, width: f32) {
        todo!()
    }

    fn get_width(&self) -> f32 {
        todo!()
    }

    fn set_name(&mut self, name: &'static str) {
        todo!()
    }

    fn get_name(&self) -> &str {
        todo!()
    }

}

// Additional functions for calculating perimeter and area for Triangle
impl Shape for Triangle {
    fn perimeter(&self) -> f32 {
        self.side_a + self.side_b + self.side_c
    }

    fn area(&self) -> f32 {
        // Using Heron's formula for the area of a triangle
        let s = self.perimeter() / 2.0;
        (s * (s - self.side_a) * (s - self.side_b) * (s - self.side_c)).sqrt()
    }

    fn new(length: f32, width: f32, name: &'static str) -> Self {
        todo!()
    }

    fn set_length(&mut self, length: f32) {
        todo!()
    }

    fn get_length(&self) -> f32 {
        todo!()
    }

    fn set_width(&mut self, width: f32) {
        todo!()
    }

    fn get_width(&self) -> f32 {
        todo!()
    }

    fn set_name(&mut self, name: &'static str) {
        todo!()
    }

    fn get_name(&self) -> &str {
        todo!()
    }
}

// Implementation of PartialEq and PartialOrd for Circle based on area
impl PartialEq for Circle {
    fn eq(&self, other: &Self) -> bool {
        //checks if the areas are equal
        self.perimeter() == other.perimeter()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Circle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.perimeter().partial_cmp(&other.perimeter())
    
    }
}

// Implementation of PartialEq and PartialOrd for Triangle based on area
impl PartialEq for Triangle {
    fn eq(&self, other: &Self) -> bool {
        self.perimeter() == other.perimeter()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Triangle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.perimeter().partial_cmp(&other.perimeter())
    }
}

// Conversion implementation into String for Circle
impl From<&'static str> for Circle {
    fn from(s: &'static str) -> Circle {
        let mut parts = s.split(',');
        let radius = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0.0,
        };
        Circle { radius }
    }
}

// Conversion implementation into String for Triangle
impl From<&'static str> for Triangle {
    fn from(s: &'static str) -> Triangle {
        let mut parts = s.split(',');
        let side_a = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0.0,
        };
        let side_b = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0.0,
        };
        let side_c = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0.0,
        };
        Triangle {
            side_a,
            side_b,
            side_c,
        }
    }
}

// Example usage
pub fn run3() {
    let circle1 = Circle { radius: 7.0 };
    let circle2 = Circle { radius: 7.0 };
    

    let triangle1 = Triangle {
        side_a: 3.0,
        side_b: 4.0,
        side_c: 5.0,
    };
    let triangle2 = Triangle {
        side_a: 5.0,
        side_b: 12.0,
        side_c: 13.0,
    };
    let triangle3 = Triangle::from("3.0,4.5,4.5");
    let circle3= Circle::from("5");


    println!("Perimeter of circle3 is :{:?}", circle3.perimeter());
    println!("Perimeter of triangle3 is : {:?}", triangle3.perimeter());

    // Compare circles and triangles based on area using PartialOrd
    let result_circle = circle1.partial_cmp(&circle2);
    let result_circle1 = circle2.partial_cmp(&circle3);
    println!("circle1 Perimeter: {:?} , cicle2 Perimeter : {:?}", circle1.perimeter(), circle2.perimeter() );
    println!("Circle Comparison: {:?}", result_circle);
    println!("Circle Comparison: {:?}", result_circle1);

    let result_triangle = triangle1.partial_cmp(&triangle2);
    let result_triangle1 = triangle3.partial_cmp(&triangle2);
    println!("triangle1 Perimeter: {:?} , triangle2 Perimeter : {:?}", triangle1.perimeter(), triangle2.perimeter() );
    println!("Triangle Comparison: {:?}", result_triangle);
    println!("Triangle Comparison: {:?}", result_triangle1);

    //Compare circles and triangles based on area using PartialEq
    let result5 = circle1.eq(&circle2);
    println!("Circle Comparison: {:?}", result5);

    let result6 = triangle1.eq(&triangle2);
    println!("Triangle Comparison: {:?}", result6);
}
