use std::ops::Index;

//we could genericize all of this and make it simpler. and slower.
// opengl really only supports 2, 3, and 4 dimensional vectors and matrices

struct Vector2
{
    elements: [f32; 2],
}

impl Vector2
{
    pub fn new(x: f32, y: f32) -> Vector2
    {
        Vector2 {
            elements: [x, y],
        }
    }

    pub fn zero() -> Vector2
    {
        Vector2 {
            elements: [0, 0],
        }
    }

    pub fn dot(&self, other: Vector2) -> f32
    {
        self.elements[0] * other.elements[0] + self.elements[1] * other.elements[1];
    }

    pub fn add(&self, other: Vector2) -> Vector2
    {
        Vector2 {
            elements: [self.elements[0] + other.elements[0], self.elements[1] + other.elements[1]],
        }
    }

    pub fn sub(&self, other: Vector2) -> Vector2
    {
        Vector2 {
            elements: [self.elements[0] - other.elements[0], self.elements[1] - other.elements[1]],
        }
    }

    pub fn mul(&self, other: f32) -> Vector2
    {
        Vector2 {
            elements: [self.elements[0] * other, self.elements[1] * other],
        }
    }
}

impl Index<usize> for Vector2
{
    type Output = f32;
    fn index(&self, i: usize) -> &f32
    {
        &self.elements[i]
    }
}

struct Vector3
{
    elements: [f32; 3],
}

impl Vector3
{
    pub fn new(x: f32, y: f32, z: f32) -> Vector3
    {
        Vector3 {
            elements: [x, y, z],
        }
    }

    pub fn zero() -> Vector3
    {
        Vector3 {
            elements: [0, 0, 0],
        }
    }

    pub fn dot(&self, other: Vector3) -> f32
    {
        self.elements[0] * other.elements[0] + self.elements[1] * other.elements[1] + self.elements[2] * other.elements[2];
    }

    pub fn add(&self, other: Vector3) -> Vector3
    {
        Vector3 {
            elements: [self.elements[0] + other.elements[0], self.elements[1] + other.elements[1], self.elements[2] + other.elements[2]],
        }
    }
}

impl Index<usize> for Vector3
{
    type Output = f32;
    fn index(&self, i: usize) -> &f32
    {
        &self.elements[i]
    }
}

struct Vector4
{
    elements: [f32; 4],
}

impl Vector4
{
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vector4
    {
        Vector4 {
            elements: [x, y, z, w],
        }
    }

    pub fn zero() -> Vector4
    {
        Vector4 {
            elements: [0, 0, 0, 0],
        }
    }

    pub fn dot(&self, other: Vector4) -> f32
    {
        self.elements[0] * other.elements[0] + self.elements[1] * other.elements[1] + self.elements[2] * other.elements[2] + self.elements[3] * other.elements[3];
    }

    pub fn add(&self, other: Vector4) -> Vector4
    {
        Vector4 {
            elements: [self.elements[0] + other.elements[0], self.elements[1] + other.elements[1], self.elements[2] + other.elements[2], self.elements[3] + other.elements[3]],
        }
    }
}

impl Index<usize> for Vector4
{
    type Output = f32;
    fn index(&self, i: usize) -> &f32
    {
        &self.elements[i]
    }
}

struct Bivector2
{
    elements: f32,
}

impl Bivector2
{
    pub fn new(xy: f32) -> Bivector2
    {
        Bivector2 {
            elements: xy,
        }
    }

    pub fn zero() -> Bivector2
    {
        Bivector2 {
            elements: 0,
        }
    }
}

struct Bivector3
{
    elements: [f32; 3],
}

impl Bivector3
{
    pub fn new(xy: f32, xz: f32, yz: f32) -> Bivector3
    {
        Bivector3 {
            elements: [xy, xz, yz],
        }
    }

    pub fn zero() -> Bivector3
    {
        Bivector3 {
            elements: [0, 0, 0],
        }
    }
}

struct Bivector4
{
    elements: [f32; 6],
}

impl Bivector4
{
    pub fn new(xy: f32, xz: f32, xw: f32, yz: f32, yw: f32, zw: f32) -> Bivector4
    {
        Bivector4 {
            elements: [xy, xz, xw, yz, yw, zw],
        }
    }

    pub fn zero() -> Bivector4
    {
        Bivector4 {
            elements: [0, 0, 0, 0, 0, 0],
        }
    }
}

struct Trivector3
{
    elements: f32,
}

impl Trivector3
{
    pub fn new(xyz: f32) -> Trivector3
    {
        Trivector3 {
            elements: xyz,
        }
    }

    pub fn zero() -> Trivector3
    {
        Trivector3 {
            elements: 0,
        }
    }
}

struct Trivector4
{
    elements: [f32; 4],
}

impl Trivector4
{
    pub fn new(xyz: f32, xyw: f32, xzw: f32, yzw: f32) -> Trivector4
    {
        Trivector3 {
            elements: [xyz, xyw, xzw, yzw],
        }
    }

    pub fn zero() -> Trivector3
    {
        Trivector3 {
            elements: [0, 0, 0, 0],
        }
    }
}

struct Matrix22
{
    elements: [Vector2; 2],
}

impl Matrix22
{
    pub fn mul_vec(&self, vec: Vector2) -> Vector2
    {
        Vector2 {
            elements: [
                vec.dot(self.elements[0]), 
                vec.dot(self.elements[1]),
                ],
        }
    }
}

impl Index<usize> for Matrix22
{
    type Output = Vector2;
    fn index(&self, i: usize) -> &Vector2
    {
        &self.elements[i]
    }
}

struct Matrix23
{
    elements: [Vector2; 3],
}

impl Matrix23
{
    pub fn mul_vec(&self, vec: Vector2) -> Vector3
    {
        Vector3 {
            elements: [
                vec.dot(self.elements[0]), 
                vec.dot(self.elements[1]), 
                vec.dot(self.elements[2]),
                ],
        }
    }
}

impl Index<usize> for Matrix23
{
    type Output = Vector2;
    fn index(&self, i: usize) -> &Vector2
    {
        &self.elements[i]
    }
}

struct Matrix24
{
    elements: [Vector2; 4],
}

impl Matrix24
{
    pub fn mul_vec(&self, vec: Vector2) -> Vector4
    {
        Vector4 {
            elements: [
                vec.dot(self.elements[0]), 
                vec.dot(self.elements[1]), 
                vec.dot(self.elements[2]), 
                vec.dot(self.elements[3]),
                ],
        }
    }
}

impl Index<usize> for Matrix24
{
    type Output = Vector2;
    fn index(&self, i: usize) -> &Vector2
    {
        &self.elements[i]
    }
}

struct Matrix32
{
    elements: [Vector3; 2],
}

impl Matrix32
{
    pub fn mul_vec(&self, vec: Vector3) -> Vector2
    {
        Vector2 {
            elements: [
                vec.dot(self.elements[0]), 
                vec.dot(self.elements[1]),
                ],
        }
    }
}

impl Index<usize> for Matrix32
{
    type Output = Vector3;
    fn index(&self, i: usize) -> &Vector3
    {
        &self.elements[i]
    }
}

struct Matrix33
{
    elements: [Vector3; 3],
}

impl Matrix33
{
    pub fn mul_vec(&self, vec: Vector3) -> Vector3
    {
        Vector3 {
            elements: [
                vec.dot(self.elements[0]), 
                vec.dot(self.elements[1]), 
                vec.dot(self.elements[2]),
                ],
        }
    }
}

impl Index<usize> for Matrix33
{
    type Output = Vector3;
    fn index(&self, i: usize) -> &Vector3
    {
        &self.elements[i]
    }
}

struct Matrix34
{
    elements: [Vector3; 4],
}

impl Matrix34
{
    pub fn mul_vec(&self, vec: Vector3) -> Vector4
    {
        Vector4 {
            elements: [
                vec.dot(self.elements[0]), 
                vec.dot(self.elements[1]), 
                vec.dot(self.elements[2]), 
                vec.dot(self.elements[3]),
                ],
        }
    }
}

impl Index<usize> for Matrix34
{
    type Output = Vector3;
    fn index(&self, i: usize) -> &Vector3
    {
        &self.elements[i]
    }
}

struct Matrix42
{
    elements: [Vector4; 2],
}

impl Matrix42
{
    pub fn mul_vec(&self, vec: Vector4) -> Vector2
    {
        Vector2 {
            elements: [
                vec.dot(self.elements[0]), 
                vec.dot(self.elements[1]),
                ],
        }
    }
}

impl Index<usize> for Matrix42
{
    type Output = Vector4;
    fn index(&self, i: usize) -> &Vector4
    {
        &self.elements[i]
    }
}

struct Matrix43
{
    elements: [Vector4; 3],
}

impl Matrix43
{
    pub fn mul_vec(&self, vec: Vector4) -> Vector3
    {
        Vector3 {
            elements: [
                vec.dot(self.elements[0]), 
                vec.dot(self.elements[1]), 
                vec.dot(self.elements[2]),
                ],
        }
    }
}

impl Index<usize> for Matrix43
{
    type Output = Vector4;
    fn index(&self, i: usize) -> &Vector4
    {
        &self.elements[i]
    }
}

struct Matrix44
{
    elements: [Vector4; 4],
}

impl Matrix44
{
    pub fn mul_vec(&self, vec: Vector4) -> Vector4
    {
        Vector4 {
            elements: [
                vec.dot(self.elements[0]), 
                vec.dot(self.elements[1]), 
                vec.dot(self.elements[2]), 
                vec.dot(self.elements[3])
                ],
        }
    }
}

impl Index<usize> for Matrix44
{
    type Output = Vector4;
    fn index(&self, i: usize) -> &Vector4
    {
        &self.elements[i]
    }
}