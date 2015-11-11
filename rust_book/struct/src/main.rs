//src: https://doc.rust-lang.org/nightly/book/structs.html

#[derive(Debug)]
struct Point {
	x: i32,
	y: i32,
}

fn main() {
	println!("Hello, struct world!");
	let origin = Point { y: 2, x: 1 }; // origin: Point
	let o1 = Point { x:1, y:2 };
	//    o1.x=3;//immutable! not allowed, good!
	//	let o2 = Point { 1,2 };//not allowed, good!
	println!("The origin is at ({}, {})", origin.x, origin.y);
	println!("{:?}",o1);

	let mut point = Point { x: 0, y: 0 };
	point.x = 5;
	println!("The point is at ({}, {})", point.x, point.y);
	let point=point; //make immutable now
	//    point.x=6; //not working anymore!
	println!("The point is at ({}, {})", point.x, point.y);

	//update syntax: https://doc.rust-lang.org/nightly/book/structs.html#update-syntax
	#[derive(Debug)]
	struct Point3d {
		x: i32,
		y: i32,
		z: i32,
	}

	let mut point = Point3d { x: 3, y: 4, z: 5 };
	point = Point3d { y: 1, .. point };
    println!("{:?}", point);//3,1,5

    let origin = Point3d { x: 9, y: 10, z: 11 };
    let point = Point3d { z: 1, x: 2, .. origin };
    println!("{:?}", point);//2,10,1
}
