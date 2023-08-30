pub mod shape {
    pub trait Area {
        fn calculate_area(&self) -> u32;
    }
    #[derive(Debug)]
    pub struct Rectangle {
        width: u32,
        height: u32,
    }
 
    impl Rectangle {
        pub fn set_w_h(self: &mut Rectangle, w: u32, h: u32) {
            self.width = w;
            self.height = h;
        }
 
        pub fn new() -> Rectangle {
            Rectangle {
                width: 0,
                height: 0,
            }
        }
    }
 
    impl Area for Rectangle {
        fn calculate_area(&self) -> u32 {
            self.height * self.width
        }
    }
    #[derive(Debug)]
    pub struct Circle {
        radius: u32,
    }
 
    impl Circle {
        pub fn set_radius(self: &mut Circle, val: u32) {
            self.radius = val;
        }
 
        pub fn new() -> Circle {
            Circle { radius: 0 }
        }
    }
 
    impl Area for Circle {
        fn calculate_area(&self) -> u32 {
            self.radius * self.radius * 3
        }
    }
}
