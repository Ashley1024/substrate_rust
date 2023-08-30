pub mod shape {
    pub trait Area {
        fn area_format(&self) -> f32;
    }
    #[derive(Debug)]
    pub struct Rectangle {
        width: f32,
        height: f32,
    }
    impl Rectangle {
        pub fn set_w_h(self: &mut Rectangle, w: f32, h: f32) {
            self.width = w;
            self.height = h;
        }
 
        pub fn new() -> Rectangle {
            Rectangle {
                width: 0.0,
                height: 0.0,
            }
        }
    }
    impl Area for Rectangle {
        fn area_format(&self) -> f32 {
            self.height * self.width
        }
    }

    #[derive(Debug)]
    pub struct Circle {
        radius: f32,
    }
 
    impl Circle {
        pub fn set_radius(self: &mut Circle, val: f32) {
            self.radius = val;
        }
 
        pub fn new() -> Circle {
            Circle { radius: 0.0 }
        }
    }
 
    impl Area for Circle {
        fn area_format(&self) -> f32 {
            self.radius * self.radius * 3.14
        }
    }

    pub struct Triangle{
        width: f32,
        height: f32,
    }
    impl Triangle {
        pub fn set_w_h(self: &mut Triangle, w: f32, h: f32) {
            self.width = w;
            self.height = h;
        }
 
        pub fn new() -> Triangle {
            Triangle {
                width: 0.0,
                height: 0.0,
            }
        }
    }
    impl Area for Triangle {
        fn area_format(&self) -> f32 {
            self.height * self.width * 0.5
        }
    }
}
