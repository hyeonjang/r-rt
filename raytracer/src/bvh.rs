// extern
use rxmath::vector::*;

use crate::bbox::*;

#[repr(packed(32))]
pub struct Node {
    min:vec3,
    max:vec3,
    second_or_index:u32,
    parent:u32, 
    axis:u32,
}

impl Node {
    pub fn new() -> Self {
        Node { min:vec3::min(), max:vec3::max(), second_or_index:u32::default(), parent:u32::default(), axis:u32::default() }
    }
}

struct BuildNode {
    bound : BBox,
    nprim : u32,
    axis : u32,
    l : Option<Box<BuildNode>>,
    r : Option<Box<BuildNode>>, 
}

impl BuildNode {
    pub fn init_leaf() {

    }
    pub fn init_interior(&mut self, axis:u32, c0:Box<BuildNode>, c1:Box<BuildNode>) {
        self.bound = BBox::bbox(c0.bound, c1.bound);
        self.l = Some(c0);
        self.r = Some(c1);
        self.nprim = 0;
        self.axis = axis;
    }
}

struct Primitive {
    id:i32,
    center:vec3,
    bound:BBox, 
}

struct BVH {
    pub nodes:Vec<Node>,
}

impl BVH {
    pub fn intersect() -> bool { return true; }
    pub fn build() {
    }
    pub fn partition() {}
}

#[cfg(test)]
mod tests {

use crate::bvh::*;

    #[test]
    fn it_works() {
        println!("[raytracer-bvh] testing module");

        let one_node = Node::new();
        assert_eq!(32, std::mem::size_of_val(&one_node));
    }
}