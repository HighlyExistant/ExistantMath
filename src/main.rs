use existant_core::{Addition, BasicField, Field, Module, Multiplication, Operator, Ring, Semiring, VectorSpace};
use existant_geoalg::{complex::Complex, matrix::{Matrix2x3, Matrix3x2}, vectors::{GeometricAlgebra, GrassmanAlgebra, InnerProductSpace, NormedVectorSpace, Vector2, Vector3}};

fn is_vector_space<F: BasicField, T: VectorSpace<Multiplication, Addition, Scalar = F>>(t: T) {
    
}
fn is_module<F: Ring<Multiplication, Addition>, T: Module<Multiplication, Addition, Scalar = F>>(t: T) {
    
}

fn main() {
    let vector = Vector3::new(1.0, 0.0, 0.0);
    let vector2 = Vector3::new(0.0, 1.0, 0.0);
    let vector3 = Vector3::new(0.0, 0.0, 1.0);
    let matrix2x3 = Matrix2x3::new(Vector3::new(1.0, 0.0, 0.0), Vector3::new(2.0, 3.0, 1.0));
    let matrix3x2 = Matrix3x2::new(Vector2::new(2.0, 3.0), Vector2::new(1.0, 0.0), Vector2::new(1.0, 0.0));
    println!("{}", vector.wedge_product(vector2));
    println!("{}", vector.wedge_product(vector2).wedge_product(vector3));
    println!("{:#?}", matrix2x3*matrix3x2);
}
