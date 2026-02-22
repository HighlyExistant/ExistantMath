use core::f32;
use std::{ops::Div, time::{Duration, Instant}};

use existant_core::{Addition, BasicField, Field, Module, Multiplication, Operator, Ring, Semiring, VectorSpace};
use existant_geoalg::{animation::remap, geometry::{FRay2D, LinearSegment2D, Ray2D, Rect2D}, matrix::{FMat4, Matrix2x3, Matrix3x2, Matrix4x4}, rotors::{Complex, Quaternion}, vectors::{FVec2, GeometricAlgebra, GrassmanAlgebra, InnerProductSpace, NormedVectorSpace, Vector2, Vector3, Vector4}};
use existant_structures::cpu::{BVH2D, RawHeap};
use image::{ImageBuffer, Rgb, RgbImage, Rgba, RgbaImage};

fn is_vector_space<F: BasicField, T: VectorSpace<Multiplication, Addition, Scalar = F>>(t: T) {
    
}
fn is_module<F: Ring<Multiplication, Addition>, T: Module<Multiplication, Addition, Scalar = F>>(t: T) {
    
}

fn main() {
    // let random_rects = (0..254usize).into_iter().map(|i|{
    //     let rect = Rect2D::new(
    //         Vector2::new(rand::random_range(10.0..50.0f32), rand::random_range(10.0..50.0)), 
    //         Vector2::new(rand::random_range(0.0..50.0f32), rand::random_range(0.0..50.0))
    //     );
    //     rect
    // }).collect::<Vec<_>>();
    // let bvh = BVH2D::new(random_rects);
    // let option = bvh.traverse(|traverse|{
    //     match traverse {
    //         existant_structures::cpu::BVH2DNodeTraverse::Leaf(leaf) => {

    //         }
    //         existant_structures::cpu::BVH2DNodeTraverse::Node(node) => {

    //         }
    //     }
    // });
    // println!("{}", 256u32.ilog2());
    // println!("{}", bvh.rects.is_leaf(256));
    let ray = FRay2D::from_angle(f32::consts::PI.div(4.0), FVec2::new(-1.0, -1.0));
    let rect = Rect2D::new(FVec2::new(-0.5, -0.500000000), FVec2::new(0.5, 0.5));
    println!("{:#?}", ray.rect_intersection(rect));
}