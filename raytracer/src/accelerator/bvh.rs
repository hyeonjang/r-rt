// extern
use rxmath::vector::*;

use crate::accelerator::*;
use bbox::*;

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

#[derive(Default)]
pub struct BuildNode {
    bound : BBox,
    nprim : u32,
    axis : u32,
    l : Option<Box<BuildNode>>,
    r : Option<Box<BuildNode>>, 
}

impl BuildNode {
    pub fn init_leaf(&mut self, first:i32, b:BBox) {
        
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
    id:usize,
    bound:BBox, 
}

pub struct BVH {
    pub nodes:Vec<Node>,
}

pub enum Method
{
    SAH,
}

impl BVH 
{
     fn build(&self, start:u32, end:u32) -> Box<BuildNode>
     {
        let mut n:Box<BuildNode> = Box::new(BuildNode::default());
        n.init_leaf(0, n.bound);
        let middle = self.partition(None);
        n.init_interior(0, self.build(start, middle), self.build(middle, end));
        return n;
     }
     pub fn partition(&self, method:Option<Method>) -> u32
     {
         match method 
         {
             None => return self.sah(),
             Some(i) => return self.sah(),
         }
     }
     pub fn sah(&self) -> u32
     {
         return 0;
     }
}

impl Accelerator for BVH 
{
    type Output = *const BuildNode;

    fn hit() -> bool 
    {
        return true;
    }
    fn build(&self, primitive:&Vec<Box<dyn Shape>>) -> Self::Output 
    {
        let mut primitives:Vec<Primitive> = Vec::with_capacity(self.nodes.capacity());
        for i in 0..primitives.len() 
        {
            primitives[i] = Primitive{ id:i, bound:primitives[i].bound };
        }
        let root:*const BuildNode = &BuildNode::default();
        return root;
    }
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