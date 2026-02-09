use existant_core::{Addition, BasicField, Field, Module, Multiplication, Operator, Ring, Semiring, VectorSpace};
use existant_geoalg::{complex::{Complex, Quaternion}, matrix::{Matrix2x3, Matrix3x2}, vectors::{GeometricAlgebra, GrassmanAlgebra, InnerProductSpace, NormedVectorSpace, Vector2, Vector3}};

fn is_vector_space<F: BasicField, T: VectorSpace<Multiplication, Addition, Scalar = F>>(t: T) {
    
}
fn is_module<F: Ring<Multiplication, Addition>, T: Module<Multiplication, Addition, Scalar = F>>(t: T) {
    
}

fn main() {
    let complex = Complex::new(1.0, 2.0);
    let quaternion = Quaternion::new(1.0, 2.0, 2.0, 1.0);
    let quaternion2 = Quaternion::new(2.0, 4.0, 3.0, 13.0);
    println!("{}", quaternion*complex);
    println!("{}", complex*quaternion);
    println!("{}", quaternion*quaternion2);
    println!("{}", quaternion2*quaternion);
}
