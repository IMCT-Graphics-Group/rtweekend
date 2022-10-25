use crate::*;
use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Matrix4x4 {
    pub m: [[Float; 4]; 4],
}

impl Default for Matrix4x4 {
    fn default() -> Self {
        Matrix4x4 {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
}

impl Matrix4x4{
    pub fn new(
        t00: Float,
        t01: Float,
        t02: Float,
        t03: Float,
        t10: Float,
        t11: Float,
        t12: Float,
        t13: Float,
        t20: Float,
        t21: Float,
        t22: Float,
        t23: Float,
        t30: Float,
        t31: Float,
        t32: Float,
        t33: Float,
    ) -> Self {
        Matrix4x4 {
            m: [
                [t00, t01, t02, t03],
                [t10, t11, t12, t13],
                [t20, t21, t22, t23],
                [t30, t31, t32, t33],
            ],
        }
    }

    pub fn transpose(m: &Matrix4x4) -> Matrix4x4 {
        Matrix4x4 {
            m: [
                [m.m[0][0], m.m[1][0], m.m[2][0], m.m[3][0]],
                [m.m[0][1], m.m[1][1], m.m[2][1], m.m[3][1]],
                [m.m[0][2], m.m[1][2], m.m[2][2], m.m[3][2]],
                [m.m[0][3], m.m[1][3], m.m[2][3], m.m[3][3]],
            ],
        }
    }

    pub fn inverse(m: &Matrix4x4) -> Matrix4x4 {
        let mut indxc = vec![0; 4];
        let mut indxr = vec![0; 4];
        let mut ipiv = vec![0; 4];
        let mut minv: Matrix4x4 = Matrix4x4::new(
            m.m[0][0], m.m[0][1], m.m[0][2], m.m[0][3], m.m[1][0], m.m[1][1], m.m[1][2], m.m[1][3],
            m.m[2][0], m.m[2][1], m.m[2][2], m.m[2][3], m.m[3][0], m.m[3][1], m.m[3][2], m.m[3][3],
        );
        for i in 0..4 {
            let mut irow = 0;
            let mut icol = 0;
            let mut big: Float = 0.0;
            // choose pivot
            for j in 0..4 {
                if ipiv[j] != 1 {
                    for (k, item) in ipiv.iter().enumerate().take(4) {
                        if *item == 0 {
                            let abs: Float = (minv.m[j][k]).abs();
                            if abs >= big {
                                big = abs;
                                irow = j;
                                icol = k;
                            }
                        } else if *item > 1 {
                            println!("Singular matrix in MatrixInvert");
                        }
                    }
                }
            }
            ipiv[icol] += 1;
            // swap rows _irow_ and _icol_ for pivot
            if irow != icol {
                for k in 0..4 {
                    // C++: std::swap(minv[irow][k], minv[icol][k]);
                    let swap = minv.m[irow][k];
                    minv.m[irow][k] = minv.m[icol][k];
                    minv.m[icol][k] = swap;
                }
            }
            indxr[i] = irow;
            indxc[i] = icol;
            if minv.m[icol][icol] == 0.0 {
                println!("Singular matrix in MatrixInvert");
            }
            // set $m[icol][icol]$ to one by scaling row _icol_ appropriately
            let pivinv: Float = 1.0 / minv.m[icol][icol];
            minv.m[icol][icol] = 1.0;
            for j in 0..4 {
                minv.m[icol][j] *= pivinv;
            }
            // subtract this row from others to zero out their columns
            for j in 0..4 {
                if j != icol {
                    let save: Float = minv.m[j][icol];
                    minv.m[j][icol] = 0.0;
                    for k in 0..4 {
                        minv.m[j][k] -= minv.m[icol][k] * save;
                    }
                }
            }
        }
        // swap columns to reflect permutation
        for i in 0..4 {
            let j = 3 - i;
            if indxr[j] != indxc[j] {
                for k in 0..4 {
                    // C++: std::swap(minv[k][indxr[j]], minv[k][indxc[j]]);
                    minv.m[k].swap(indxr[j], indxc[j])
                }
            }
        }
        minv
    }
}

impl PartialEq for Matrix4x4 {
    fn eq(&self, rhs: &Matrix4x4) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                if self.m[i][j] != rhs.m[i][j] {
                    return false;
                }
            }
        }
        true
    }
}

/// 两矩阵相乘
pub fn mtx_mul(m1: &Matrix4x4, m2: &Matrix4x4) -> Matrix4x4 {
    let mut r: Matrix4x4 = Matrix4x4::default();
    for i in 0..4 {
        for j in 0..4 {
            r.m[i][j] = m1.m[i][0] * m2.m[0][j]
                + m1.m[i][1] * m2.m[1][j]
                + m1.m[i][2] * m2.m[2][j]
                + m1.m[i][3] * m2.m[3][j];
        }
    }
    r
}

#[derive(Debug, Copy, Clone)]
pub struct Transform {
    pub m: Matrix4x4,
    pub m_inv: Matrix4x4,
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            m: Matrix4x4::default(),
            m_inv: Matrix4x4::default(),
        }
    }
}

impl Transform {
    pub fn new(
        t00: Float,
        t01: Float,
        t02: Float,
        t03: Float,
        t10: Float,
        t11: Float,
        t12: Float,
        t13: Float,
        t20: Float,
        t21: Float,
        t22: Float,
        t23: Float,
        t30: Float,
        t31: Float,
        t32: Float,
        t33: Float,
    ) -> Self {
        Transform {
            m: Matrix4x4::new(
                t00, t01, t02, t03, t10, t11, t12, t13, t20, t21, t22, t23, t30, t31, t32, t33,
            ),
            m_inv: Matrix4x4::inverse(&Matrix4x4::new(
                t00, t01, t02, t03, t10, t11, t12, t13, t20, t21, t22, t23, t30, t31, t32, t33,
            )),
        }
    }

    pub fn inverse(t: &Transform) -> Transform {
        Transform {
            m: t.m_inv,
            m_inv: t.m,
        }
    }

    pub fn is_identity(&self) -> bool {
        self.m.m[0][0] == 1.0 as Float
            && self.m.m[0][1] == 0.0 as Float
            && self.m.m[0][2] == 0.0 as Float
            && self.m.m[0][3] == 0.0 as Float
            && self.m.m[1][0] == 0.0 as Float
            && self.m.m[1][1] == 1.0 as Float
            && self.m.m[1][2] == 0.0 as Float
            && self.m.m[1][3] == 0.0 as Float
            && self.m.m[2][0] == 0.0 as Float
            && self.m.m[2][1] == 0.0 as Float
            && self.m.m[2][2] == 1.0 as Float
            && self.m.m[2][3] == 0.0 as Float
            && self.m.m[3][0] == 0.0 as Float
            && self.m.m[3][1] == 0.0 as Float
            && self.m.m[3][2] == 0.0 as Float
            && self.m.m[3][3] == 1.0 as Float
    }

    pub fn swaps_handedness(&self) -> bool {
        let det: Float = self.m.m[0][0]
            * (self.m.m[1][1] * self.m.m[2][2] - self.m.m[1][2] * self.m.m[2][1])
            - self.m.m[0][1] * (self.m.m[1][0] * self.m.m[2][2] - self.m.m[1][2] * self.m.m[2][0])
            + self.m.m[0][2] * (self.m.m[1][0] * self.m.m[2][1] - self.m.m[1][1] * self.m.m[2][0]);
        det < 0.0 as Float
    }

    pub fn translate(delta: &Vec3) -> Transform {
        Transform {
            m: Matrix4x4::new(
                1.0, 0.0, 0.0, delta.x(), 0.0, 1.0, 0.0, delta.y(), 0.0, 0.0, 1.0, delta.z(), 0.0, 0.0,
                0.0, 1.0,
            ),
            m_inv: Matrix4x4::new(
                1.0, 0.0, 0.0, -delta.x(), 0.0, 1.0, 0.0, -delta.y(), 0.0, 0.0, 1.0, -delta.z(), 0.0,
                0.0, 0.0, 1.0,
            ),
        }
    }

    pub fn scale(x:Float, y:Float, z:Float) -> Transform{
        Transform {
            m: Matrix4x4::new(
                x, 0.0, 0.0, 0.0, 0.0, y, 0.0, 0.0, 0.0, 0.0, z, 0.0, 0.0, 0.0, 0.0, 1.0,
            ),
            m_inv: Matrix4x4::new(
                1.0 / x,
                0.0,
                0.0,
                0.0,
                0.0,
                1.0 / y,
                0.0,
                0.0,
                0.0,
                0.0,
                1.0 / z,
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
            ),
        }
    }

    pub fn rotate_x(theta: Float) -> Transform {
        let sin_theta: Float = degree_to_radians(theta).sin();
        let cos_theta: Float = degree_to_radians(theta).cos();
        let m = Matrix4x4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, cos_theta, -sin_theta, 0.0, 0.0, sin_theta, cos_theta, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );
        Transform {
            m,
            m_inv: Matrix4x4::transpose(&m),
        }
    }

    pub fn rotate_y(theta: Float) -> Transform {
        let sin_theta: Float = degree_to_radians(theta).sin();
        let cos_theta: Float = degree_to_radians(theta).cos();
        let m = Matrix4x4::new(
            cos_theta, 0.0, sin_theta, 0.0, 0.0, 1.0, 0.0, 0.0, -sin_theta, 0.0, cos_theta, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );
        Transform {
            m,
            m_inv: Matrix4x4::transpose(&m),
        }
    }

    pub fn rotate_z(theta: Float) -> Transform {
        let sin_theta: Float = degree_to_radians(theta).sin();
        let cos_theta: Float = degree_to_radians(theta).cos();
        let m = Matrix4x4::new(
            cos_theta, -sin_theta, 0.0, 0.0, sin_theta, cos_theta, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );
        Transform {
            m,
            m_inv: Matrix4x4::transpose(&m),
        }
    }

    pub fn rotate(theta: Float, axis: &Vec3) -> Transform {
        let a: Vec3 = axis.unit_vector();
        let sin_theta: Float = degree_to_radians(theta).sin();
        let cos_theta: Float = degree_to_radians(theta).cos();
        let mut m = Matrix4x4::default();
        // compute rotation of first basis vector
        m.m[0][0] = a.0 * a.0 + (1.0 - a.0 * a.0) * cos_theta;
        m.m[0][1] = a.0 * a.1 * (1.0 - cos_theta) - a.2 * sin_theta;
        m.m[0][2] = a.0 * a.2 * (1.0 - cos_theta) + a.1 * sin_theta;
        m.m[0][3] = 0.0;
        // compute rotations of second basis vectors
        m.m[1][0] = a.0 * a.1 * (1.0 - cos_theta) + a.2 * sin_theta;
        m.m[1][1] = a.1 * a.1 + (1.0 - a.1 * a.1) * cos_theta;
        m.m[1][2] = a.1 * a.2 * (1.0 - cos_theta) - a.0 * sin_theta;
        m.m[1][3] = 0.0;
        // compute rotations of third basis vectors
        m.m[2][0] = a.0 * a.2 * (1.0 - cos_theta) - a.1 * sin_theta;
        m.m[2][1] = a.1 * a.2 * (1.0 - cos_theta) + a.0 * sin_theta;
        m.m[2][2] = a.2 * a.2 + (1.0 - a.2 * a.2) * cos_theta;
        m.m[2][3] = 0.0;
        Transform {
            m,
            m_inv: Matrix4x4::transpose(&m),
        }
    }

    pub fn look_at(pos: &Point3, look: &Point3, up: &Vec3) -> Transform {
        let mut camera_to_world = Matrix4x4::default();
        // initialize fourth column of viewing matrix
        camera_to_world.m[0][3] = pos.0;
        camera_to_world.m[1][3] = pos.1;
        camera_to_world.m[2][3] = pos.2;
        camera_to_world.m[3][3] = 1.0;
        // initialize first three columns of viewing matrix
        let dir: Vec3 = (*look - *pos).unit_vector();
        if Vec3::cross_borrow(&up.unit_vector(), &dir).length() == 0.0 {
            println!(
                "\"up\" vector ({}, {}, {}) and viewing direction ({}, {}, {}) passed to \
                 LookAt are pointing in the same direction.  Using the identity \
                 transformation.",
                up.0, up.1, up.2, dir.0, dir.1, dir.2
            );
            Transform::default()
        } else {
            let left: Vec3 = Vec3::cross_borrow(&up.unit_vector(), &dir).unit_vector();
            let new_up: Vec3 = Vec3::cross_borrow(&dir, &left);
            camera_to_world.m[0][0] = left.0;
            camera_to_world.m[1][0] = left.1;
            camera_to_world.m[2][0] = left.2;
            camera_to_world.m[3][0] = 0.0;
            camera_to_world.m[0][1] = new_up.0;
            camera_to_world.m[1][1] = new_up.1;
            camera_to_world.m[2][1] = new_up.2;
            camera_to_world.m[3][1] = 0.0;
            camera_to_world.m[0][2] = dir.0;
            camera_to_world.m[1][2] = dir.1;
            camera_to_world.m[2][2] = dir.2;
            camera_to_world.m[3][2] = 0.0;
            Transform {
                m: Matrix4x4::inverse(&camera_to_world),
                m_inv: camera_to_world,
            }
        }
    }

    pub fn orthographic(z_near: Float, z_far: Float) -> Transform {
        let translate: Transform = Transform::translate(&Vec3 (
            0.0,
            0.0,
            -z_near,
        ));
        let scale: Transform = Transform::scale(1.0, 1.0, 1.0 / (z_far - z_near));
        scale * translate
    }

    pub fn perspective(fov: Float, n: Float, f: Float) -> Transform {
        // perform projective divide for perspective projection
        let persp = Matrix4x4::new(
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            f / (f - n),
            -f * n / (f - n),
            0.0,
            0.0,
            1.0,
            0.0,
        );
        // scale canonical perspective view to specified field of view
        let inv_tan_ang: Float = 1.0 / (degree_to_radians(fov) / 2.0).tan();
        let scale: Transform = Transform::scale(inv_tan_ang, inv_tan_ang, 1.0);
        let persp_trans: Transform = Transform {
            m: persp,
            m_inv: Matrix4x4::inverse(&persp),
        };
        scale * persp_trans
    }

    pub fn transform_point(&self, p: &Point3) -> Point3 {
        let x: Float = p.0;
        let y: Float = p.1;
        let z: Float = p.2;
        let xp: Float =
            self.m.m[0][0] * x + self.m.m[0][1] * y + self.m.m[0][2] * z + self.m.m[0][3];
        let yp: Float =
            self.m.m[1][0] * x + self.m.m[1][1] * y + self.m.m[1][2] * z + self.m.m[1][3];
        let zp: Float =
            self.m.m[2][0] * x + self.m.m[2][1] * y + self.m.m[2][2] * z + self.m.m[2][3];
        let wp: Float =
            self.m.m[3][0] * x + self.m.m[3][1] * y + self.m.m[3][2] * z + self.m.m[3][3];
        assert!(wp != 0.0, "wp = {:?} != 0.0", wp);
        if wp == 1.0 as Float {
            Point3::new_point3(
                xp, 
                yp, 
                zp
            )
        } else {
            let inv: Float = 1.0 as Float / wp;
            Point3::new_point3(
                inv * xp,
                inv * yp,
                inv * zp,
            )
        }
    }

    pub fn transform_vector(&self, v: &Vec3) -> Vec3 {
        let x: Float = v.0;
        let y: Float = v.1;
        let z: Float = v.2;
        Vec3 (
            self.m.m[0][0] * x + self.m.m[0][1] * y + self.m.m[0][2] * z,
            self.m.m[1][0] * x + self.m.m[1][1] * y + self.m.m[1][2] * z,
            self.m.m[2][0] * x + self.m.m[2][1] * y + self.m.m[2][2] * z,
        )
    }
    
    pub fn transform_normal(&self, n: &Vec3) -> Vec3 {
        let x: Float = n.0;
        let y: Float = n.1;
        let z: Float = n.2;
        Vec3 (
            self.m_inv.m[0][0] * x + self.m_inv.m[1][0] * y + self.m_inv.m[2][0] * z,
            self.m_inv.m[0][1] * x + self.m_inv.m[1][1] * y + self.m_inv.m[2][1] * z,
            self.m_inv.m[0][2] * x + self.m_inv.m[1][2] * y + self.m_inv.m[2][2] * z,
        )
    }

}

impl PartialEq for Transform {
    fn eq(&self, rhs: &Transform) -> bool {
        rhs.m == self.m && rhs.m_inv == self.m_inv
    }
}

impl Mul for Transform {
    type Output = Transform;
    fn mul(self, rhs: Transform) -> Transform {
        Transform {
            m: mtx_mul(&self.m, &rhs.m),
            m_inv: mtx_mul(&rhs.m_inv, &self.m_inv),
        }
    }
}