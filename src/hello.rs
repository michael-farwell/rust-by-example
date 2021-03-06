#![allow(unused)]

use std::cmp::min;
use std::f32::consts::PI;
use std::fmt::{self, Display, Formatter};

pub fn hello_world() {
    println!("Hello, world!");
    println!("I'm a Rustacean!");
}

pub fn formatted_print() {
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}.", "Alice", "Bob");
    println!("{subject} {verb} {object}",
             object = "the lazy dog",
             subject = "the quick brown fox",
             verb = "jumps over");
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
    println!("{number:>width$}", number = 1, width = 6);
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // #[allow(dead_code)]
    // struct Structure(i32);
    // println!("This struct `{}` won't print...", Structure(3));

    println!("Pi is roughly {:.3}", PI);
}

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

pub fn debug() {
    println!("{:?} months in a year", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor = "actor's");
    println!("Now {:?} will print!", Structure(3));
    println!("Now {:?} will print!", Deep(Structure(7)));

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    println!("{:#?}", peter)
}

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

pub fn display() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    let complex = Complex { real: 3.3, imag: 7.2 };

    println!("Compare complex:");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex)
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}", count, v)?;
        }

        write!(f, "]")
    }
}

pub fn testcase() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v)
}

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        write!(f, "{}: {:.3}??{} {:.3}??{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "RGB ({}, {}, {}) 0x{:02X}{:02X}{:02X}",
               self.red, self.green, self.blue,
               self.red, self.green, self.blue)
    }
}

pub fn formatting() {
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", city)
    };

    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{}", *color);
        println!("{:?}", *color)
    }
}