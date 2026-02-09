mod mat2x2;
mod mat3x2;
mod mat2x3;
mod mat3x3;
mod mat3x4;
mod mat2x4;
mod mat4x3;
mod mat4x4;
use existant_core::{Module, Ring, Semimodule, Semiring};
pub use mat2x2::*;
pub use mat3x2::*;
pub use mat2x3::*;
pub use mat3x3::*;
pub use mat3x4::*;
pub use mat2x4::*;
pub use mat4x3::*;
pub use mat4x4::*;

pub trait Matrix 
    where <Self::Vector as Semimodule>::Scalar: Ring {
    type Vector: Module;
    type TransposeMatrix;
    fn transpose(&self) -> Self::TransposeMatrix;
    
}
pub trait SquareMatrix: Matrix<TransposeMatrix = Self> {
    fn minor(&self, column: usize, row: usize) -> <Self::Vector as Semimodule>::Scalar;
    fn cofactor(&self, column: usize, row: usize) -> <Self::Vector as Semimodule>::Scalar;
    fn cofactor_matrix(&self) -> Self;
    fn adjoint(&self) -> Self 
        where Self: Sized {
        self.cofactor_matrix().transpose()
    }
    fn determinant(&self) -> <Self::Vector as Semimodule>::Scalar;
}

pub trait MatrixSemiring: Matrix 
    where <<Self as Matrix>::Vector as Semimodule>::Scalar: Semiring {
    
}
impl<M: Matrix> MatrixSemiring for M 
    where <<Self as Matrix>::Vector as Semimodule>::Scalar: Semiring {
    
}

pub trait MatrixRing: Matrix
    where <<Self as Matrix>::Vector as Semimodule>::Scalar: Ring {
    
}
impl<M: Matrix> MatrixRing for M 
    where <<Self as Matrix>::Vector as Semimodule>::Scalar: Ring {
    
}
/// This trait is used for structures that can be used
/// to represent a family of equations such as those
/// that implement the [`Matrix`] trait. This specifically
/// can apply to those matrices which are (n+1)xn.
pub trait SolveEquations: Sized {
    fn solve_system(&self) -> Option<Self>;
}