use std::ops::Mul;
pub struct Vec3{
    pub x:f32,
    pub y:f32,
    pub z:f32,
}
impl Vec3{
    pub fn single_val(val:f32)->Self{
        Self{
            x:val,
            y:val,
            z:val
        }
    }
    pub fn new(x:f32,y:f32,z:f32)->Self{
        Self{
            x,
            y,
            z
        }
    }
    pub fn length_squared(&self)->f32{
        return (self.x*self.x)+(self.y*self.y)+(self.z*self.z);
    }
    pub fn length(&self) ->f32{
        return self.length_squared().sqrt();
    }
}
impl Mul for Vec3{
      type Output = Self;
    fn mul(self,rhs:Vec3)->Self {
        let x = self.x *rhs.x;
        let y = self.y * rhs.y;
        let z = self.z * rhs.z;
        Self::new(x,y,z)
    }  
}