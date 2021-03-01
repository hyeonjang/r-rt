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
    //SAH,
    EQUAL,
    //MIDDLE,
}

///////////////////////////////////////////////////////
/// 2. BVH build information
#[derive(Clone)]
struct Primitive {
    id:i32,
    center:vec3,
    bound:Bounds3f,
}

type PrimitiveList = Vec<Primitive>;

#[derive(Default)]
struct BuildNode {
    bound : Bounds3f,
    center : Bounds3f,
    id : i32,
    axis : usize,
    l : Option<Box<BuildNode>>,
    r : Option<Box<BuildNode>>, 
}

impl BuildNode {
    fn is_leaf(&self) -> bool {
        return self.axis == 3;
    }
    fn init_leaf(&self, id:i32, b:Bounds3f) -> Self {
        BuildNode { bound:b, center:b, id:id, axis:3, l:None, r:None }
    }
    fn init_interior(&mut self, axis:usize, c0:Box<BuildNode>, c1:Box<BuildNode>) {
        self.bound = Bounds3f::bounds(c0.bound, c1.bound);
        self.l = Some(c0);
        self.r = Some(c1);
        self.id = -1;
        self.axis = axis;
    }
}

struct BVHBuild {
    method : Method,
    unordered_prmitives : PrimitiveList,
}

impl BVHBuild {
    fn new(primitives:&PrimitiveList, method:Method) -> Self {
        BVHBuild{ unordered_prmitives:primitives.to_vec(), method:method }        
    }

    // fn split_sah() -> usize {
    //     return 0;
    // }

    fn split_equal_counts(primitives:&mut PrimitiveList, start:usize, end:usize, dim:usize) -> usize {
        let mid = (start+end)/2;
        primitives.select_nth_unstable_by(mid, |p0, p1| p0.center[dim].partial_cmp(&p1.center[dim]).unwrap() );
        return mid;
    }

    fn build(&mut self, start:usize, end:usize) -> Box<BuildNode> {
        let mut n:Box<BuildNode> = Box::new(BuildNode::default());

        let bnd_prim = &mut n.bound;
        let bnd_cent = &mut n.center;
        for i in start..end {
            bnd_prim.expand(self.unordered_prmitives[i].bound);
            bnd_cent.expand(self.unordered_prmitives[i].center);
        }

        let dim = bnd_cent.max_extend() as usize;
        let nprim = end-start;

        if nprim==1 || bnd_cent.max[dim]==bnd_cent.min[dim] {
            return Box::new(n.init_leaf(self.unordered_prmitives[start].id, n.bound));
        }

        let mid = match self.method {
            //Method::SAH => BVHBuild::split_sah(),
            Method::EQUAL => BVHBuild::split_equal_counts(&mut self.unordered_prmitives, start, end, dim),
            //Method::MIDDLE => BVHBuild::split_sah(),
        };

        n.init_interior(dim, self.build(start, mid), self.build(mid, end));
        return n;
     }
}

////////////////////////
/// 3. Real BVH
#[derive(Clone)]
pub struct Node {
    bound: Bounds3f,
    second:isize,
    parent:u32,
    id:usize,
    axis:u32,
}

impl Node {
    pub fn new() -> Self {
        Node { bound:Bounds3f::default(), id:usize::default(), second:isize::default(), parent:u32::default(), axis:u32::default() }
    }
    pub fn is_leaf(&self) -> bool {
        return self.axis == 3;
    }
}

pub struct BVH {
    nodes:Vec<Node>,
    mesh:*mut ShapeList,
}

impl BVH {
    pub fn new(mesh:*mut ShapeList) -> Self {
        BVH { nodes:vec![], mesh:mesh }
    }
    pub fn alloc_node(&mut self) {
        self.nodes.push(Node::new());
    }
    fn flatten_tree(&mut self, node:Option<&Box<BuildNode>>, offset:&mut isize) -> isize {
        let node = match node {
            Some(_) => node.unwrap(),
            None => return 0,
        };

        *offset = *offset+1;
        let offset0 = *offset;

        self.alloc_node();
        self.nodes[*offset as usize].second = node.id as isize;
        self.nodes[*offset as usize].bound  = node.bound;
        self.nodes[*offset as usize].axis   = node.axis as u32;
        
        if !node.is_leaf() {
            self.flatten_tree(node.l.as_ref(), offset);
            self.nodes[offset0 as usize].second = self.flatten_tree(node.r.as_ref(), offset);
        }
        return offset0;
    }
}

impl Accelerator for BVH {
    fn build(&mut self, primitive:&Vec<Box<dyn Shape>>) {
        
        let mut primitives:Vec<Primitive> = Vec::with_capacity(primitive.len());
     
        for i in 0..primitives.capacity() {
            primitives.push(Primitive{ id:i as i32, bound:primitive[i].bounds(), center:primitive[i].bounds().center() });
            println!("{} {}", primitives[i].center, primitives[i].bound);
        }

        let mut bvh_build = BVHBuild::new(&primitives, Method::EQUAL);

        let len = primitives.len();
        let root = bvh_build.build(0, len);
        
        BVH::flatten_tree(self, Some(&root), &mut -1);

        println!("\nCheck data");
        for n in &self.nodes {
            println!("{}", n.bound);
            println!("axis:{} second:{}", n.axis, n.second);
        }

    }
}

impl Intersect for BVH {
    fn intersect(&self, r:&Ray, t_min:f32, t_max:f32, h:&mut Hit) -> bool {
        if self.nodes.is_empty() { return false; }
        let mut hit = false;
        let inv_d = vec3(1.0/r.d.x, 1.0/r.d.y, 1.0/r.d.z);
        let d_neg = vec3(inv_d.x<0.0, inv_d.y<0.0, inv_d.z<0.0); 
        let mut to = 1;
        let mut stack = [0;64];
        while to != 0 {
            to -= 1;
            let idx = stack[to];
            let node = &self.nodes[idx];
            if !node.is_leaf() {
                if node.bound.intersect(r, t_min, t_max, h) {
                    stack[to + d_neg[node.axis as usize] as usize] = idx+1;
                    stack[to + !d_neg[node.axis as usize] as usize] = node.second as usize;
                    to += 2;
                }
                continue;
            }
            unsafe {
                let mut i = Hit::default();
                if (*self.mesh).list[node.second as usize].intersect(r, t_min, t_max, &mut i) && i.t_min<h.t_min {
                    *h = i;
                    hit=true;
                }
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

}