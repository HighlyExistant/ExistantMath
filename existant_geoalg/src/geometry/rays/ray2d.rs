use existant_core::{Addition, BasicField, FloatingPoint, Identity, Multiplication, Semimodule, UniversalOperationsOn};

use crate::{geometry::{Intersect, LinearSegment2D, RayIntersection, Rect2D}, vectors::{GrassmanAlgebra, InnerProductSpace, MetricSpace, NormedVectorSpace, Vector2}};

// pub struct RectRayIntersection2D<T> {
//     intersections: [Vector2<T>; 2],
// }

pub struct Ray2D<T> {
    dir: Vector2<T>,
    pos: Vector2<T>
}

impl<T: BasicField> Ray2D<T> 
    where Vector2<T>: NormedVectorSpace + Semimodule<Scalar = T> {
    
    /// Creates a new [`Ray2D`] object. Assumes that the `dir`
    /// provided is normalized, for speed.
    pub unsafe fn new_unchecked(dir: Vector2<T>, pos: Vector2<T>) -> Self {
        Self { dir, pos }
    }
    pub fn new(dir: Vector2<T>, pos: Vector2<T>) -> Self {
        Self { dir: dir.normalize(), pos }
    }
    pub fn new_look(look_at: Vector2<T>, pos: Vector2<T>) -> Self {
        Self { dir: (look_at-pos).normalize(), pos }
    }
    pub fn from_angle(angle: T, pos: Vector2<T>) -> Self 
        where T: FloatingPoint {
        Self { dir: Vector2::new(angle.cos(), angle.sin()).normalize(), pos }
    }
    pub unsafe fn set_dir_unchecked(&mut self, dir: Vector2<T>) {
        self.dir = dir
    }
    pub unsafe fn set_dir(&mut self, dir: Vector2<T>) {
        self.dir = dir.normalize()
    }
    pub fn set_pos(&mut self, pos: Vector2<T>) {
        self.pos = pos;
    }
    #[inline]
    pub fn pos(&self) -> Vector2<T> {
        self.pos
    }
    #[inline]
    pub fn dir(&self) -> Vector2<T> {
        self.dir
    }
    /// Intersection between a ray and a line formed by
    /// the equation `ax+b`. Inspired by [this stackoverflow question](https://stackoverflow.com/questions/14307158/how-do-you-check-for-intersection-between-a-line-segment-and-a-line-ray-emanatin/32146853#32146853)
    pub fn line_intersection(&self, segment: LinearSegment2D<T>)  -> Option<RayIntersection<T>>
        where T: FloatingPoint + BasicField {
        let ray_seg_dir = self.pos-segment.points[0];
        let segment_dir = segment.points[1]-segment.points[0];
        let perp_ray = self.dir.perpendicular();

        // If the vectors are perpendicular, then it means they
        // never intersect. When the dot product is zero, the
        // two vectors are perpendicular.
        let dot = segment_dir.inner_product(perp_ray);
        if dot.abs() < T::from_f32(f32::EPSILON) {
            return None;
        }

        let t1 = segment_dir.wedge_product(ray_seg_dir)/dot;
        let t2 = ray_seg_dir.inner_product(perp_ray)/dot;

        if t1 >= <T as Identity<Addition>>::IDENTITY && (t2 >= <T as Identity<Addition>>::IDENTITY && t2 <= <T as Identity<Multiplication>>::IDENTITY) {
            return Some(RayIntersection { 
                point: self.dir*t1+self.pos, 
                distance: t1
            });
        }

        None
    }
    pub fn rect_intersection(&self, rect: Rect2D<T>) -> Option<RayIntersection<T>> 
        where T: BasicField + UniversalOperationsOn<T> + FloatingPoint {
        let min = rect.min();
        let max = rect.max();
        let minpos = min-self.pos;
        let maxpos = max-self.pos;
        let tan = self.dir.tan();
        let cot = self.dir.cot();
        // Find the intersections for all the segments of the rectangle,
        // formed by the equations.
        // 1. x = min.x
        // 2. x = max.x
        // 3. y = min.y
        // 4. y = max.y
        let ix0 = self.pos.x + (minpos.y)*cot;
        let ix1 = self.pos.x + (maxpos.y)*cot;
        let iy0 = self.pos.y + (minpos.x)*tan;
        let iy1 = self.pos.y + (maxpos.x)*tan;
        
        // if ix1 > iy0 || ix0 < iy1 {
        //     return None;
        // }

        let mut min_dist_point = <Vector2<T> as Identity<Addition>>::IDENTITY;
        let mut min_dist = T::MAX;

        let perpdir = self.dir.perpendicular();

        if min.x <= ix0 && ix0 <= max.x {
            let intersect = Vector2::new(ix0, min.y);
            if perpdir.wedge_product(intersect - self.pos) < <T as Identity<Addition>>::IDENTITY {
                let dist = self.pos.distance(intersect);
                if dist < min_dist {
                    min_dist = dist;
                    min_dist_point = intersect;
                }
            }
        }
        if min.x <= ix1 && ix1 <= max.x {
            let intersect = Vector2::new(ix1, max.y);
            if perpdir.wedge_product(intersect - self.pos) < <T as Identity<Addition>>::IDENTITY {
                let dist = self.pos.distance(intersect);
                if dist < min_dist {
                    min_dist = dist;
                    min_dist_point = intersect;
                }
            }
        } 
        if min.y <= iy0 && iy0 <= max.y {
            let intersect = Vector2::new(min.x, iy0);
            if perpdir.wedge_product(intersect - self.pos) < <T as Identity<Addition>>::IDENTITY {
                let dist = self.pos.distance(intersect);
                if dist < min_dist {
                    min_dist = dist;
                    min_dist_point = intersect;
                }
            }
        }
        if min.y <= iy1 && iy1 <= max.y {
            let intersect = Vector2::new(max.x, iy1);
            if perpdir.wedge_product(intersect - self.pos) < <T as Identity<Addition>>::IDENTITY {
                let dist = self.pos.distance(intersect);
                if dist < min_dist {
                    min_dist = dist;
                    min_dist_point = intersect;
                }
            }
        }

        if min_dist != T::MAX {
            return Some(RayIntersection { point: min_dist_point, distance: min_dist }); 
        }
        None
    }
    pub fn is_rect_intersecting(&self, rect: Rect2D<T>) -> bool
        where T: BasicField + UniversalOperationsOn<T> + FloatingPoint {
        let min = rect.min();
        let max = rect.max();
        let minpos = min-self.pos;
        let maxpos = max-self.pos;
        let tan = self.dir.tan();
        let cot = self.dir.cot();
        let ix0 = self.pos.x + (minpos.y)*cot;
        let ix1 = self.pos.x + (maxpos.y)*cot;
        let iy0 = self.pos.y + (minpos.x)*tan;
        let iy1 = self.pos.y + (maxpos.x)*tan;

        let perpdir = self.dir.perpendicular();


        if min.x <= ix0 && ix0 <= max.x {
            let intersect = Vector2::new(ix0, min.y);
            if perpdir.wedge_product(intersect - self.pos) < <T as Identity<Addition>>::IDENTITY {
                return true;
            }
        }
        if min.x <= ix1 && ix1 <= max.x {
            let intersect = Vector2::new(ix1, max.y);
            if perpdir.wedge_product(intersect - self.pos) < <T as Identity<Addition>>::IDENTITY {
                return true;
            }
        } 
        if min.y <= iy0 && iy0 <= max.y {
            let intersect = Vector2::new(min.x, iy0);
            if perpdir.wedge_product(intersect - self.pos) < <T as Identity<Addition>>::IDENTITY {
                return true;
            }
        }
        if min.y <= iy1 && iy1 <= max.y {
            let intersect = Vector2::new(max.x, iy1);
            if perpdir.wedge_product(intersect - self.pos) < <T as Identity<Addition>>::IDENTITY {
                return true;

            }
        }

        false
    }
}

impl<T: FloatingPoint + BasicField> Intersect<LinearSegment2D<T>> for Ray2D<T> {
    type Scalar = T;
    fn intersect(&self, with: &LinearSegment2D<T>) -> Option<super::RayIntersection<T>> {
        self.line_intersection(*with)
    }
}

pub type FRay2D = Ray2D<f32>;
pub type DRay2D = Ray2D<f64>;