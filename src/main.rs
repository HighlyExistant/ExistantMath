#![allow(unused)]
use core::f32;
use std::{fmt::Debug, ops::{Deref, DerefMut, Div, Mul}, time::{Duration, Instant}};

use existant_core::{Addition, BasicField, Field, Module, Multiplication, Operator, Ring, Semiring, UniversalOperationsOn, VectorSpace};
use existant_geoalg::{animation::remap, derivative::Derivative, geometry::{FRay2D, LinearSegment2D, Ray2D, Rect2D, Sphere2D, VertexShape}, matrix::{FMat4, Matrix2x2, Matrix2x3, Matrix3x2, Matrix4x4}, rotors::{Complex, Quaternion}, vectors::{FVec2, FVec4, GeometricAlgebra, GrassmanAlgebra, InnerProductSpace, NormedVectorSpace, Vector2, Vector3, Vector4}};
use image::{ImageBuffer, Rgb, RgbImage, Rgba, RgbaImage};

fn is_vector_space<F: BasicField, T: VectorSpace<Multiplication, Addition, Scalar = F>>(t: T) {
    
}
fn is_module<F: Ring<Multiplication, Addition>, T: Module<Multiplication, Addition, Scalar = F>>(t: T) {
    
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tagged<T> {
    value: T,
    tag: &'static str,
}

trait Taggable: Sized {
    fn tag(self, tag: &'static str) -> Tagged<Self> {
        Tagged { value: self, tag }
    }
}
impl<T, Q> From<Tagged<T>> for Rect2D<Q> 
    where Rect2D<Q>: From<T>,
    Q: UniversalOperationsOn<Q> {
    fn from(value: Tagged<T>) -> Self {
        Rect2D::<Q>::from(value.value)
    }
}
impl<T> Taggable for T {}

impl<T> Deref for Tagged<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl<T> DerefMut for Tagged<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T> Debug for Tagged<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.tag)
    }
}

fn main() {
    let rect = Rect2D::new(FVec2::new(0.0, 0.0), FVec2::new(320.0, 200.0));
    let perspective = Matrix4x4::perspective(
        50.0f32.to_radians(), 
        320.0/200.0, 
        20.0, 
        0.1
    );//*Matrix4x4::perspective_view(Vector3::new(0.0, 0.0, 0.0), Vector3::default());
    println!("{:#?}", perspective);
    let point = FVec2::new(12.0, 14.0);
    println!("{}", point.to_vec4(0.0, 1.0));
    println!("{}", perspective.mul(point.to_vec4(0.01, 1.0)));
    println!("{}", perspective.mul(point.to_vec4(0.5, 1.0)));
}