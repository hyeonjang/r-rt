// extern
use rxmath::vector::*;

use crate::accelerator::*;
use bounds::*;

use crate::intersect::*;
use ray::*;
use hit::*;

////////////////////////////////////////////////////
/// 1. Interface and essential BVH information.
pub enum Method {
    SAH,
    EQUAL,
    MIDDLE,
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
struct BuildNode {
    bound : Bounds,
    center : Bounds,
    nprim : i32,
    axis : usize,
    l : Option<Box<BuildNode>>,
    r : Option<Box<BuildNode>>, 
}

impl BuildNode {
    fn is_leaf(&self) -> bool {
        return self.nprim > 0;
    }
    fn init_leaf(&self, n:i32, b:Bounds) -> Self {
        BuildNode { bound:b, center:b, nprim:n, axis:3, l:None, r:None }
    }
    fn init_interior(&mut self, axis:usize, c0:Box<BuildNode>, c1:Box<BuildNode>) {
        self.bound = Bounds::bounds(c0.bound, c1.bound);
        self.l = Some(c0);
        self.r = Some(c1);
        self.nprim = 0;
        self.axis = axis;
    }
    fn return_id(&self) -> i32 {
        self.nprim
    }
}

struct BVHBuild {
    method : Method,
    unordered_prmitives : PrimitiveList,
    ordered_primitives : PrimitiveList,
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

    fn build(&mut self, start:usize, end:usize) -> Box<BuildNode> {
        let mut n:Box<BuildNode> = Box::new(BuildNode::default());

        let b_prim = &mut n.bound;
        let b_cent = &mut n.center;
        for i in start..end {
            b_prim.expand(self.unordered_prmitives[i].bound);
            b_cent.expand(self.unordered_prmitives[i].center);
        }

        let dim = b_cent.max_extend() as usize;
        let nprim = end - start;

        if nprim==1 || b_cent.max[dim]==b_cent.min[dim] {
            self.ordered_primitives.push(self.unordered_prmitives[start].clone());
            return Box::new(n.init_leaf(self.unordered_prmitives[start].id, n.bound));
        }

        let mid = match self.method {
            Method::SAH => BVHBuild::split_sah(),
            Method::EQUAL => BVHBuild::split_equal_counts(&mut self.unordered_prmitives, start, end, dim),
            Method::MIDDLE => BVHBuild::split_sah(),
        };

        n.init_interior(dim, self.build(start, mid), self.build(mid, end));
        return n;
     }
}

////////////////////////
/// 3. Real BVH
#[derive(Clone)]
pub struct Node {
    bound: Bounds,
    second:usize,
    parent:u32, 
    axis:u32,
}

impl Node {
    pub fn new() -> Self {
        Node { bound:Bounds::default(), second:usize::default(), parent:u32::default(), axis:u32::default() }
    }
    pub fn is_leaf(&self) -> bool {
        return self.axis == 3;
    }
}

pub struct BVH {
    pub nodes:Vec<Node>,
    pub mesh:*mut ShapeList,
}

impl BVH {
    pub fn new(mesh:*mut ShapeList) -> Self {
        BVH { nodes:vec![], mesh:mesh }
    }
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
    pub fn resize(&mut self, len:usize) {
        self.nodes.clear(); self.nodes.resize(len, Node::new());
    }
    pub fn capacity(&self) -> usize {
        self.nodes.capacity()
    }
    pub fn flatten_tree(&mut self, node:Option<&Box<BuildNode>>, offset:&mut isize) -> isize {
        let node = match node {
            Some(s) => node.unwrap(),
            None => return 0, //@@todo primitivie id
        };

        *offset = *offset + 1;
        let offset0 = *offset;

        self.nodes[*offset as usize].bound = node.bound;
        self.nodes[*offset as usize].axis = node.axis as u32;
        
        if !node.is_leaf() {
            self.flatten_tree(node.l.as_ref(), offset);
            self.nodes[offset0 as usize].second = self.flatten_tree(node.r.as_ref(), offset) as usize;
        }
        return offset0;
    }
}

impl Accelerator for BVH {
    fn build(&mut self, primitive:&Vec<Box<dyn Shape>>) {
        
        let mut primitives:Vec<Primitive> = Vec::with_capacity(primitive.len());
        
        for i in 0..primitives.capacity() {
            primitives.push(Primitive{ id:i as i32, bound:primitive[i].bounds(), center:primitive[i].bounds().center() });
        }

        let mut bvh_build = BVHBuild::new(&primitives, Method::EQUAL);

        let len = primitives.len();
        let root = bvh_build.build(0, len);

        self.resize(5); //@@todo
        BVH::flatten_tree(self, Some(&root), &mut -1);
    }
}

////////////////////////////////////////////
impl Intersect for BVH {
    fn intersect(&self, r:&Ray, h:&mut Hit) -> bool {
        if self.nodes.is_empty() { return false; }
        let mut hit = false;
        let mut idx = 0;
        let mut to = 0;
        let invDir = vec3(1.0/r.dir.x, 1.0/r.dir.y, 1.0/r.dir.z);
        let dirIsNeg = vec3(invDir.x<0.0, invDir.y<0.0, invDir.z<0.0); 
        let mut stack = [0;64];
        loop {
            let mut node = &self.nodes[idx];
            if node.bound.intersect(r, h) {
                if node.is_leaf() {
                    unsafe {
                        if (*self.mesh).list[node.second].intersect(r, h) {
                            hit = true;
                            if to <= 0 { break; }
                            to -= 1;
                            idx = stack[to];
                        }
                    }
                } else {
                    if dirIsNeg[node.axis as usize] {
                        to += 1;
                        stack[to] = idx + 1;
                        idx = node.second;
                    }
                    else {
                        to += 1;
                        stack[to] = node.second;
                        idx = idx + 1;
                    }
                }
            } else {
                if to == 0 { break; }
                to -= 1;
                idx = stack[to];
            }
        }
        return hit;
    }
}

#[allow(non_snake_case)]
pub fn Create(mesh:*mut ShapeList, primitives:&Vec<Box<dyn Shape>>) -> Box<BVH> {
    let mut bvh = BVH::new(mesh);
    bvh.build(primitives);
    return Box::new(bvh);
}


#[cfg(test)]
mod tests { 

// use crate::bvh::*;

//     #[test]
//     fn it_works() {
//         println!("[raytracer-bvh] testing module");

//         let one_node = Node::new();
//         assert_eq!(32, std::mem::size_of_val(&one_node));
//     }
}