use rusttracer::vec3::Vec3;
fn main() {
    let vec = Vec3{
        x:10.0,
        y:20.0,
        z:30.0
    };
    let vector = vec * Vec3::single_val(2.0);
    println!("{} {} {}",vector.x,vector.y,vector.z);
    println!("{}",vector.length_squared());
    //generate_image();
}
fn generate_image(){
    let width = 256;
    let height = 256;
    let rgbspace: f32 = 255.999;
    print!("P3\n {} {} \n255\n",width,height);
    for x_coord in (0..width).rev(){
        eprintln!("Scanlines remaining: {}",x_coord);
        for y_coord in 0..height{
            
            let r:f32 = y_coord as f32 / (width as f32 -1.0);
            let g: f32 = x_coord as f32 / (height as f32 - 1.0);
            let ir:i32 = (rgbspace * r) as i32;
            let ig = (rgbspace * g) as i32;
            let ib = (0.25 * rgbspace)as i32;
            print!("{} {} {}\n",ir,ig,ib);
        }
    }
}