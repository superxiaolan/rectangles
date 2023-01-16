# [derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self,other:&Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
//同一个结构体可以有多个impl块
impl Rectangle {
    //关联函数不需要接收self作为参数（因为一般方法需要接收self作为第一个参数），
    //却仍然可以把这种函数跟结构体关联起来，所以称为关联函数
    //关联函数可以将那些不需要实例的特定功能放置到结构体的命名空间里
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    /*let rect1 = Rectangle {
        width: 30,
        height: 50
    };*/

    /*println!("The area of the rectangle is {} square pixels.",
    area(&rect1)
    );*/

    let rect1 = Rectangle { width: 30, height: 50};
    let rect2 = Rectangle { width: 10, height: 40};
    let rect3 = Rectangle { width: 60, height: 45};

    let sq = Rectangle::square(9);
    /*println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);
    println!("The area of the rectangle is {} square pixels.",
             rect1.area()//不需要实参声明，因为self会调用方法的结构体实例
    );*/
    println!("The area of the rectangle is {} square pixels.",
             rect1.area()//不需要实参声明，因为self会调用方法的结构体实例
    );
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect3));

    println!("rect1 is {:#?}", sq);
}
