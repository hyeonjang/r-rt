// extern
use rxmath::vector::*;

use crate::accelerator::*;
use bounds::*;

////////////////////////////////////////////////////
/// 1. Interface and essential BVH information.
pub enum Method{
    SAH,
    EQUAL,
}
 
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

pub struct BVH {
    pub nodes:Vec<Node>,
}
///
///////////////////////////////////////////////////////
/// 2. BVH build information

#[derive(Clone)]
struct Primitive {
    id:usize,
    center:vec3,
    bound:Bounds, 
}

type PrimitiveList = Vec<Primitive>;

#[derive(Default)]
struct BuildNode {
    bound : Bounds,
    //center : vec3,
    nprim : u32,
    axis : u32,
    l : Option<Box<BuildNode>>,
    r : Option<Box<BuildNode>>, 
}

impl BuildNode {
    pub fn init_leaf(&self, first:i32, b:Bounds) -> Self {
        BuildNode { bound:b, nprim:1, axis:1, l:None, r:None }
    }
    pub fn init_interior(&mut self, axis:u32, c0:Box<BuildNode>, c1:Box<BuildNode>) {
        self.bound = Bounds::bounds(c0.bound, c1.bound);
        self.l = Some(c0);
        self.r = Some(c1);
        self.nprim = 0;
        self.axis = axis;
    }
}

pub struct BVHBuild {
    unordered_prmitives : PrimitiveList,
    ordered_primitives : PrimitiveList,
    method : Method,
    //memory_pool : 
}

impl BVHBuild {
    fn new(primitives:&PrimitiveList, method:Method) -> Self {
        BVHBuild{ unordered_prmitives:primitives.to_vec(), ordered_primitives:vec![], method:method }        
    }

     fn build(&mut self, start:usize, end:usize) -> Box<BuildNode>     {
        let mut n:Box<BuildNode> = Box::new(BuildNode::default());
        let b_prim = &mut n.bound;
        //let mut b_cent = &mut n.center;
        println!("{:?} {:?}", b_prim.min, b_prim.max);
        for i in start..end {
            b_prim.expand(self.unordered_prmitives[i].bound);
            //b_cent.expand(self.unordered_prmitives[i].center);
        }
        let nprim = end - start;
        if nprim == 1 {
            //self.nodes.push();
            return Box::new(n.init_leaf(0, n.bound));
        }
        let mut middle = (start+end)/2;
        let mut dim = b_prim.max_extend() as usize;
        middle = match self.method {
            Method::SAH => self.split_sah(),
            Method::EQUAL => 
            {
                let selected = self.unordered_prmitives.select_nth_unstable_by(2, |p0, p1| p0.center[dim].partial_cmp(&p1.center[dim]).unwrap() );
                (start+end)/2
            }
        };

        n.init_interior(0, self.build(start, middle), self.build(middle, end));
        return n;
     }

     fn split_sah(&self) -> usize {
         return 0;
     }

}
///
////////////////////////////////////////////
impl Accelerator for BVH {
    fn hit(&self) -> bool {
        return true;
    }
    fn build<'a>(&self, primitive:&Vec<Box<dyn Shape + 'a>>) {
        let mut primitives:Vec<Primitive> = Vec::with_capacity(self.nodes.capacity());

        let mut bvh_build = BVHBuild::new(&primitives, Method::SAH);
        bvh_build.build(0, 10);
        //let mut unordered_prmitives = primitives;

        // for i in 0..primitives.len() 
        // {
        //     primitives[i] = Primitive{ id:i, bound:primitives[i].bound };
        // }
        let root:*const BuildNode = &BuildNode::default();
    }
}

// pub fn create(primitives:&Vec<Box<dyn Shape>>) -> BVH {
//     let bvh:BVH;

//     let unordered_prmitives = primitives;


//     return bvh;
// }


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