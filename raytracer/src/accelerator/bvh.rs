// extern
use rxmath::vector::*;

use crate::accelerator::*;
use bounds::*;

////////////////////////////////////////////////////
/// 1. Interface and essential BVH information.
pub enum Method{
    SAH,
    EQUAL,
    MIDDLE,
}
 
#[repr(packed(32))]
pub struct Node {
    min:vec3,
    max:vec3,
    second_or_index:usize,
    parent:u32, 
    axis:u32,
}

impl Node {
    pub fn new() -> Self {
        Node { min:vec3::min(), max:vec3::max(), second_or_index:usize::default(), parent:u32::default(), axis:u32::default() }
    }
}

pub struct BVH {
    pub nodes:Vec<Node>,
}

impl BVH {
    pub fn new() -> Self {
        BVH { nodes:vec![], }
    }
    pub fn flatten_tree(bvh:&mut BVH, node:Option<&Box<BuildNode>>, offset:usize) -> usize {
        let offset0 = offset+1;

        let node = match node {
            Some(s) => node.unwrap(),
            None => return 0,
        };

        BVH::flatten_tree(bvh, node.l.as_ref(), offset0);
        bvh.nodes[offset].second_or_index = BVH::flatten_tree(bvh, node.r.as_ref(), offset0);

        return offset0;
    }
}

///////////////////////////////////////////////////////
/// 2. BVH build information
#[derive(Clone)]
struct Primitive {
    id:i32,
    center:vec3,
    bound:Bounds,
}

type PrimitiveList = Vec<Primitive>;

#[derive(Default)]
pub struct BuildNode {
    bound : Bounds,
    //center : vec3,
    nprim : u32,
    axis : usize,
    l : Option<Box<BuildNode>>,
    r : Option<Box<BuildNode>>, 
}

impl BuildNode {
    fn init_leaf(&self, first:i32, b:Bounds) -> Self {
        BuildNode { bound:b, nprim:1, axis:1, l:None, r:None }
    }
    fn init_interior(&mut self, axis:usize, c0:Box<BuildNode>, c1:Box<BuildNode>) {
        self.bound = Bounds::bounds(c0.bound, c1.bound);
        self.l = Some(c0);
        self.r = Some(c1);
        self.nprim = 0;
        self.axis = axis;
    }
}

struct BVHBuild {
    unordered_prmitives : PrimitiveList,
    ordered_primitives : PrimitiveList,
    method : Method,
    //memory_pool : 
}

impl BVHBuild {
    fn new(primitives:&PrimitiveList, method:Method) -> Self {
        BVHBuild{ unordered_prmitives:primitives.to_vec(), ordered_primitives:vec![], method:method }        
    }

    fn split_sah() -> usize {
        return 0;
    }

    fn split_equal_counts(primitives:&mut PrimitiveList, start:usize, end:usize, dim:usize) -> usize {
        let selected = primitives.select_nth_unstable_by(2, |p0, p1| p0.center[dim].partial_cmp(&p1.center[dim]).unwrap() );
        return (start+end)/2;
    }

    fn build(&mut self, start:usize, end:usize) -> Box<BuildNode>     {
        let mut n:Box<BuildNode> = Box::new(BuildNode::default());
        let b_prim = &mut n.bound;
        //let mut b_cent = &mut n.center;
        for i in start..end {
            b_prim.expand(self.unordered_prmitives[i].bound);
            //b_cent.expand(self.unordered_prmitives[i].center);
        }
        let dim = b_prim.max_extend() as usize;
        let nprim = end - start;
        if nprim <= 1 {
            self.ordered_primitives.push(self.unordered_prmitives[start].clone());
            return Box::new(n.init_leaf(self.unordered_prmitives[start].id, n.bound));
        }
        let mut middle = (start+end)/2;
        middle = match self.method {
            Method::SAH => BVHBuild::split_sah(),
            Method::EQUAL => BVHBuild::split_equal_counts(&mut self.unordered_prmitives, start, end, dim),
            Method::MIDDLE => BVHBuild::split_sah(),
        };

        n.init_interior(dim, self.build(start, middle), self.build(middle, end));
        return n;
     }
}

////////////////////////////////////////////
impl Accelerator for BVH {
    fn hit(&self) -> bool {
        return true;
    }
    fn build(&mut self, primitive:&Vec<Box<dyn Shape>>) {
        for prim in primitive {
            println!("{} {}", prim.bounds().min, prim.bounds().max);
        }

        let mut primitives:Vec<Primitive> = Vec::with_capacity(primitive.capacity());
        println!("{:?}", primitives.capacity());
        for i in 0..primitives.capacity() {
            primitives.push(Primitive{ id:i as i32, bound:primitive[i].bounds(), center:primitive[i].bounds().center() });
        }
        let mut bvh_build = BVHBuild::new(&primitives, Method::EQUAL);
        let root = bvh_build.build(0, primitives.len());

        for i in bvh_build.ordered_primitives {
            println!("{}", i.bound.min);
        }

        println!("start flatten");

        BVH::flatten_tree(self, Some(&root), 0);

        for node in &self.nodes {
            println!("{}", node.second_or_index);
        }
    }
}

pub fn create(primitives:&Vec<Box<dyn Shape>>) -> Box<BVH> {
    let mut bvh = Box::new(BVH::new());
    bvh.build(primitives);

    let unordered_prmitives = primitives;

    return bvh;
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