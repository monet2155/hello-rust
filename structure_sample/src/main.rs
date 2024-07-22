fn main() {
    // 1.
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // 2.
    let rect1 = (width1, height1);

    println!(
        "The area with dimensions of the rectangle is {} square pixels.",
        area_with_dimensions(rect1)
    );

    // 3.
    let rect2 = Rectangle {
        width: width1,
        height: height1,
    };

    println!(
        "The area with structure of the rectangle is {} square pixels.",
        area_with_structure(&rect2)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_with_dimensions(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn area_with_structure(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
