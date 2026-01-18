use super::{Point3, Vector3};
use std::ops::Mul;

pub struct Matrix4 {
    elements: [f32; 16],
}

impl std::convert::From<[f32; 16]> for Matrix4 {
  fn from(elements: [f32; 16]) -> Self {
    Matrix4 { elements }
  }
}

impl Matrix4 {
    pub fn new_perspective(aspect: f32, field_of_view_in_radians: f32, near: f32, far: f32) -> Self {
      // let f = (std::f32::consts::PI * 0.5 - 0.5 * field_of_view_in_radians).tan();
      let f = 1.0 / (0.5 * field_of_view_in_radians).tan();
      let range_inv = 1.0 / (near - far);
  
      [ f / aspect, 0.0, 0.0, 0.0,
        0.0, f, 0.0, 0.0,
        0.0, 0.0, (near + far) * range_inv, -1.0,
        0.0, 0.0, near * far * range_inv * 2.0, 0.0,
      ].into()
    }

    pub fn as_slice(&self) -> &[f32] { &self.elements }
  
    pub fn new_projection(width: f32, height: f32, depth: f32) -> Self {
      // Note: This matrix flips the Y axis so 0 is at the top.
      [  2.0 / width, 0.0, 0.0, 0.0,
         0.0, -2.0 / height, 0.0, 0.0,
         0.0, 0.0, 2.0 / depth, 0.0,
        -1.0, 1.0, 0.0, 1.0,
      ].into()
    }

    pub fn multiply(&self, other: &Matrix4) -> Matrix4 {
      multiply(&self.elements, &other.elements).into()
    }
  
    pub fn new_translation(tx: f32, ty: f32, tz: f32) -> Matrix4 {
      [  1.0,  0.0,  0.0,  0.0,
         0.0,  1.0,  0.0,  0.0,
         0.0,  0.0,  1.0,  0.0,
         tx, ty, tz, 1.0,
      ].into()
    }

    pub fn look_at_rh(camera_position: &Point3, target: &Point3, up: &Vector3) -> Matrix4 {
      let z_axis = (camera_position - target).normalize();
      let x_axis = up.cross(&z_axis).normalize();
      let y_axis = z_axis.cross(&x_axis).normalize();

      inverse(&
      [ x_axis.dx(), x_axis.dy(), x_axis.dz(), 0.0,
        y_axis.dx(), y_axis.dy(), y_axis.dz(), 0.0,
        z_axis.dx(), z_axis.dy(), z_axis.dz(), 0.0,
        camera_position.x(),
        camera_position.y(),
        camera_position.z(),
        1.0,
      ]).into()
    }
  
  /*
    xRotation: function(angleInRadians) {
      var c = Math.cos(angleInRadians);
      var s = Math.sin(angleInRadians);
  
      return [
        1, 0, 0, 0,
        0, c, s, 0,
        0, -s, c, 0,
        0, 0, 0, 1,
      ];
    },
  
    yRotation: function(angleInRadians) {
      var c = Math.cos(angleInRadians);
      var s = Math.sin(angleInRadians);
  
      return [
        c, 0, -s, 0,
        0, 1, 0, 0,
        s, 0, c, 0,
        0, 0, 0, 1,
      ];
    },
  
    zRotation: function(angleInRadians) {
      var c = Math.cos(angleInRadians);
      var s = Math.sin(angleInRadians);
  
      return [
         c, s, 0, 0,
        -s, c, 0, 0,
         0, 0, 1, 0,
         0, 0, 0, 1,
      ];
    },
  
    scaling: function(sx, sy, sz) {
      return [
        sx, 0,  0,  0,
        0, sy,  0,  0,
        0,  0, sz,  0,
        0,  0,  0,  1,
      ];
    },
  
    translate: function(m, tx, ty, tz) {
      return m4.multiply(m, m4.translation(tx, ty, tz));
    },
  
    xRotate: function(m, angleInRadians) {
      return m4.multiply(m, m4.xRotation(angleInRadians));
    },
  
    yRotate: function(m, angleInRadians) {
      return m4.multiply(m, m4.yRotation(angleInRadians));
    },
  
    zRotate: function(m, angleInRadians) {
      return m4.multiply(m, m4.zRotation(angleInRadians));
    },
  
    scale: function(m, sx, sy, sz) {
      return m4.multiply(m, m4.scaling(sx, sy, sz));
    },
  
    */
}

impl Mul<&Matrix4> for &Matrix4 {
  type Output = Matrix4;
  fn mul(self, other: &Matrix4) -> Matrix4 {
    self.multiply(other)
  }
}


fn multiply(a: &[f32; 16], b: &[f32; 16]) -> [f32; 16] {
      let a00 = a[        0];
      let a01 = a[        1];
      let a02 = a[        2];
      let a03 = a[        3];
      let a10 = a[    4    ];
      let a11 = a[    4 + 1];
      let a12 = a[    4 + 2];
      let a13 = a[    4 + 3];
      let a20 = a[2 * 4    ];
      let a21 = a[2 * 4 + 1];
      let a22 = a[2 * 4 + 2];
      let a23 = a[2 * 4 + 3];
      let a30 = a[3 * 4    ];
      let a31 = a[3 * 4 + 1];
      let a32 = a[3 * 4 + 2];
      let a33 = a[3 * 4 + 3];
      let b00 = b[        0];
      let b01 = b[        1];
      let b02 = b[        2];
      let b03 = b[        3];
      let b10 = b[    4    ];
      let b11 = b[    4 + 1];
      let b12 = b[    4 + 2];
      let b13 = b[    4 + 3];
      let b20 = b[2 * 4    ];
      let b21 = b[2 * 4 + 1];
      let b22 = b[2 * 4 + 2];
      let b23 = b[2 * 4 + 3];
      let b30 = b[3 * 4    ];
      let b31 = b[3 * 4 + 1];
      let b32 = b[3 * 4 + 2];
      let b33 = b[3 * 4 + 3];
      [
        b00 * a00 + b01 * a10 + b02 * a20 + b03 * a30,
        b00 * a01 + b01 * a11 + b02 * a21 + b03 * a31,
        b00 * a02 + b01 * a12 + b02 * a22 + b03 * a32,
        b00 * a03 + b01 * a13 + b02 * a23 + b03 * a33,
        b10 * a00 + b11 * a10 + b12 * a20 + b13 * a30,
        b10 * a01 + b11 * a11 + b12 * a21 + b13 * a31,
        b10 * a02 + b11 * a12 + b12 * a22 + b13 * a32,
        b10 * a03 + b11 * a13 + b12 * a23 + b13 * a33,
        b20 * a00 + b21 * a10 + b22 * a20 + b23 * a30,
        b20 * a01 + b21 * a11 + b22 * a21 + b23 * a31,
        b20 * a02 + b21 * a12 + b22 * a22 + b23 * a32,
        b20 * a03 + b21 * a13 + b22 * a23 + b23 * a33,
        b30 * a00 + b31 * a10 + b32 * a20 + b33 * a30,
        b30 * a01 + b31 * a11 + b32 * a21 + b33 * a31,
        b30 * a02 + b31 * a12 + b32 * a22 + b33 * a32,
        b30 * a03 + b31 * a13 + b32 * a23 + b33 * a33,
      ]
}

fn inverse(m: &[f32; 16]) -> [f32; 16] {
      let m00 = m[        0];
      let m01 = m[        1];
      let m02 = m[        2];
      let m03 = m[        3];
      let m10 = m[    4    ];
      let m11 = m[    4 + 1];
      let m12 = m[    4 + 2];
      let m13 = m[    4 + 3];
      let m20 = m[2 * 4    ];
      let m21 = m[2 * 4 + 1];
      let m22 = m[2 * 4 + 2];
      let m23 = m[2 * 4 + 3];
      let m30 = m[3 * 4    ];
      let m31 = m[3 * 4 + 1];
      let m32 = m[3 * 4 + 2];
      let m33 = m[3 * 4 + 3];
      let tmp_0  = m22 * m33;
      let tmp_1  = m32 * m23;
      let tmp_2  = m12 * m33;
      let tmp_3  = m32 * m13;
      let tmp_4  = m12 * m23;
      let tmp_5  = m22 * m13;
      let tmp_6  = m02 * m33;
      let tmp_7  = m32 * m03;
      let tmp_8  = m02 * m23;
      let tmp_9  = m22 * m03;
      let tmp_10 = m02 * m13;
      let tmp_11 = m12 * m03;
      let tmp_12 = m20 * m31;
      let tmp_13 = m30 * m21;
      let tmp_14 = m10 * m31;
      let tmp_15 = m30 * m11;
      let tmp_16 = m10 * m21;
      let tmp_17 = m20 * m11;
      let tmp_18 = m00 * m31;
      let tmp_19 = m30 * m01;
      let tmp_20 = m00 * m21;
      let tmp_21 = m20 * m01;
      let tmp_22 = m00 * m11;
      let tmp_23 = m10 * m01;
  
      let t0 = (tmp_0 * m11 + tmp_3 * m21 + tmp_4 * m31) -
               (tmp_1 * m11 + tmp_2 * m21 + tmp_5 * m31);
      let t1 = (tmp_1 * m01 + tmp_6 * m21 + tmp_9 * m31) -
               (tmp_0 * m01 + tmp_7 * m21 + tmp_8 * m31);
      let t2 = (tmp_2 * m01 + tmp_7 * m11 + tmp_10 * m31) -
               (tmp_3 * m01 + tmp_6 * m11 + tmp_11 * m31);
      let t3 = (tmp_5 * m01 + tmp_8 * m11 + tmp_11 * m21) -
               (tmp_4 * m01 + tmp_9 * m11 + tmp_10 * m21);
  
      let d = 1.0 / (m00 * t0 + m10 * t1 + m20 * t2 + m30 * t3);
  
      [
        d * t0,
        d * t1,
        d * t2,
        d * t3,
        d * ((tmp_1 * m10 + tmp_2 * m20 + tmp_5 * m30) -
             (tmp_0 * m10 + tmp_3 * m20 + tmp_4 * m30)),
        d * ((tmp_0 * m00 + tmp_7 * m20 + tmp_8 * m30) -
             (tmp_1 * m00 + tmp_6 * m20 + tmp_9 * m30)),
        d * ((tmp_3 * m00 + tmp_6 * m10 + tmp_11 * m30) -
             (tmp_2 * m00 + tmp_7 * m10 + tmp_10 * m30)),
        d * ((tmp_4 * m00 + tmp_9 * m10 + tmp_10 * m20) -
             (tmp_5 * m00 + tmp_8 * m10 + tmp_11 * m20)),
        d * ((tmp_12 * m13 + tmp_15 * m23 + tmp_16 * m33) -
             (tmp_13 * m13 + tmp_14 * m23 + tmp_17 * m33)),
        d * ((tmp_13 * m03 + tmp_18 * m23 + tmp_21 * m33) -
             (tmp_12 * m03 + tmp_19 * m23 + tmp_20 * m33)),
        d * ((tmp_14 * m03 + tmp_19 * m13 + tmp_22 * m33) -
             (tmp_15 * m03 + tmp_18 * m13 + tmp_23 * m33)),
        d * ((tmp_17 * m03 + tmp_20 * m13 + tmp_23 * m23) -
             (tmp_16 * m03 + tmp_21 * m13 + tmp_22 * m23)),
        d * ((tmp_14 * m22 + tmp_17 * m32 + tmp_13 * m12) -
             (tmp_16 * m32 + tmp_12 * m12 + tmp_15 * m22)),
        d * ((tmp_20 * m32 + tmp_12 * m02 + tmp_19 * m22) -
             (tmp_18 * m22 + tmp_21 * m32 + tmp_13 * m02)),
        d * ((tmp_18 * m12 + tmp_23 * m32 + tmp_15 * m02) -
             (tmp_22 * m32 + tmp_14 * m02 + tmp_19 * m12)),
        d * ((tmp_22 * m22 + tmp_16 * m02 + tmp_21 * m12) -
             (tmp_20 * m12 + tmp_23 * m22 + tmp_17 * m02)),
      ]
}
