extern crate pbrt;

use pbrt::{Float, Point3f, Ray, Sphere, Transform, Vector3f};

fn main() {
    // see CreateSphereShape() in sphere.cpp
    let radius: Float = 1.0;
    let z_min: Float = -radius;
    let z_max: Float = radius;
    let phi_max: Float = 360.0;
    let translate: Transform = Transform::translate(Vector3f {
        x: -1.3,
        y: 0.0,
        z: 0.0,
    });
    let inverse: Transform = Transform::inverse(translate);
    let sphere: Sphere = Sphere::new(inverse,
                                     translate,
                                     false,
                                     false,
                                     radius,
                                     z_min,
                                     z_max,
                                     phi_max);
    // see Sphere::Intersect() in sphere.cpp
    let o: Point3f = Point3f {
        x: 2.0,
        y: 1.99999988,
        z: 4.99999905,
    };
    let d: Vector3f = Vector3f {
        x: -0.0607556403,
        y: -0.164096087,
        z: -0.984571517,
    };
    let r: Ray = Ray { o: o, d: d };
    let mut t_hit: Float = 0.0;
    let did_ray_interesect: bool = sphere.intersect(&r, &mut t_hit);

    println!("translate = {:?}", translate);
    println!("r = {:?}", r);
    println!("sphere.intersect(r, {:?}) = {:?}",
             t_hit,
             did_ray_interesect);
    // WORK
}