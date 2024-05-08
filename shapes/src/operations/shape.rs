use crate::utils;

pub struct Shape {
    pub(crate) name: String,
    pub(crate) area: u32,
    pub(crate) perimeter: u32,
}

impl Shape {
    pub fn new(name: String, area: u32, perimeter: u32) -> Shape {
        Shape {
            name,
            area,
            perimeter,
        }
    }
    pub fn square() -> Shape {
        let a: u32 = utils::func::get_input("Enter the side value: ");
        Shape {
            name: String::from("square"),
            area: a * a,
            perimeter: 4 * a,
        }
    }
    pub fn rect() -> Shape {
        let length: u32 = utils::func::get_input("Enter the length value: ");
        let breadth: u32 = utils::func::get_input("Enter the breadth value: ");
        Shape {
            name: String::from("Rectange"),
            area: length * breadth,
            perimeter: 2 * (length + breadth),
        }
    }
    pub fn circle() -> Shape {
        let pi: f32 = 3.14;
        let r: u32 = utils::func::get_input("Enter the radius value: ");
        Shape {
            name: String::from("Circle"),
            area: (pi * (r as f32) * (r as f32)) as u32,
            perimeter: (2.0 * pi * (r as f32)) as u32,
        }
    }
    pub fn triangle() ->Shape {
        let s1: u32 = utils::func::get_input("Enter the side 1 value: ");
        let s2: u32 = utils::func::get_input("Enter the side 2 value: ");
        let s3: u32 = utils::func::get_input("Enter the side 3 value: ");
        let s = Shape {
            name: String::from(if s1 == s2 && s2 == s3 {
                "Equilateral Triangle"
            } else if s1 + s2 < s3 || s3 + s2 < s1 || s1 + s3 < s2 {
                return Shape::new(String::from("Invalid Shape"), 0, 0);
            } else if s1 == s2 && s2 != s3 || s1 == s3 && s2 != s3 || s3 == s2 && s2 != s1 {
                "Isosceles Triangle"
            } else {
                "Scalene Triangle"
            }),
            area: 0,
            perimeter: s1 + s2 + s3,
        };
        s
    }
}
