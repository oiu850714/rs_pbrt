extern crate pbrt;

use pbrt::{AnimatedTransform, Bounds2f, Bounds2i, BoxFilter, BVHAccel, DirectLightingIntegrator,
           Film, Float, LightStrategy, PerspectiveCamera, Point2f, Point2i, Point3f, Primitive,
           Sphere, SplitMethod, Transform, Triangle, TriangleMesh, Vector2f, Vector3f,
           ZeroTwoSequenceSampler};
use std::string::String;

struct SceneDescription {
    meshes: Vec<TriangleMesh>,
    spheres: Vec<Sphere>,
}

struct SceneDescriptionBuilder {
    meshes: Vec<TriangleMesh>,
    spheres: Vec<Sphere>,
}

impl SceneDescriptionBuilder {
    fn new() -> SceneDescriptionBuilder {
        SceneDescriptionBuilder {
            meshes: Vec::new(),
            spheres: Vec::new(),
        }
    }
    fn add_mesh(&mut self,
                object_to_world: Transform,
                world_to_object: Transform,
                n_triangles: usize,
                vertex_indices: Vec<usize>,
                n_vertices: usize,
                p_ws: Vec<Point3f>,
                s: Vec<Vector3f>,
                n: Vec<Vector3f>,
                uv: Vec<Point2f>)
                -> &mut SceneDescriptionBuilder {
        let triangle_mesh: TriangleMesh = TriangleMesh::new(object_to_world,
                                                            world_to_object,
                                                            false,
                                                            false,
                                                            n_triangles,
                                                            vertex_indices,
                                                            n_vertices,
                                                            p_ws, // in world space
                                                            s, // empty
                                                            n, // empty
                                                            uv);
        println!("triangle_mesh = {:?}", triangle_mesh);
        println!("vertex_indices = {:?}", triangle_mesh.vertex_indices);
        self.meshes.push(triangle_mesh);
        self
    }
    fn add_sphere(&mut self,
                  object_to_world: Transform,
                  world_to_object: Transform,
                  radius: Float,
                  z_min: Float,
                  z_max: Float,
                  phi_max: Float)
                  -> &mut SceneDescriptionBuilder {
        let sphere: Sphere = Sphere::new(object_to_world,
                                         world_to_object,
                                         false,
                                         false,
                                         radius,
                                         z_min,
                                         z_max,
                                         phi_max);
        println!("sphere = {:?}", sphere);
        self.spheres.push(sphere);
        self
    }
    fn finalize(&self) -> SceneDescription {
        SceneDescription {
            meshes: self.meshes.to_vec(),
            spheres: self.spheres.to_vec(),
        }
    }
}

struct Scene<'scene> {
    primitives: Vec<&'scene Primitive>,
    triangles: Vec<Triangle<'scene>>,
}

impl<'s> Scene<'s> {
    fn new(scene: &'s SceneDescription) -> Scene<'s> {
        let mut primitives: Vec<&Primitive> = Vec::new();
        let mut triangles: Vec<Triangle> = Vec::new();
        // meshes
        for sphere in &scene.spheres {
            primitives.push(sphere);
        }
        // meshes
        for mesh in &scene.meshes {
            // create individual triangles
            for id in 0..mesh.n_triangles {
                let triangle: Triangle = Triangle::new(mesh.object_to_world,
                                                       mesh.world_to_object,
                                                       mesh.transform_swaps_handedness,
                                                       &mesh,
                                                       id);
                triangles.push(triangle);
                // primitives.push(&triangle);
            }
        }
        // triangles from above
        for triangle in &triangles {
            // primitives.push(triangle as &'s Primitive);
        }
        Scene {
            primitives: primitives,
            triangles: triangles,
        }
    }
}


fn main() {
    // pbrt::MakeShapes

    // trianglemesh

    // Shape "trianglemesh"  "integer indices" [0 2 1 0 3 2 ]
    let vertex_indices: Vec<usize> = vec![0_usize, 2, 1, 0, 3, 2];
    let n_triangles: usize = vertex_indices.len() / 3;
    // "point P" [-100 -1 -100 400 -1 -100 400 -1 400 -100 -1 400 ]
    let p: Vec<Point3f> = vec![Point3f {
                                   x: -100.0,
                                   y: -1.0,
                                   z: -100.0,
                               },
                               Point3f {
                                   x: 400.0,
                                   y: -1.0,
                                   z: -100.0,
                               },
                               Point3f {
                                   x: 400.0,
                                   y: -1.0,
                                   z: 400.0,
                               },
                               Point3f {
                                   x: -100.0,
                                   y: -1.0,
                                   z: 400.0,
                               }];
    // "float st" [ 0 0 1 0 0 1 1 1]
    let uv: Vec<Point2f> = vec![Point2f { x: 0.0, y: 0.0 },
                                Point2f { x: 1.0, y: 0.0 },
                                Point2f { x: 0.0, y: 1.0 },
                                Point2f { x: 1.0, y: 1.0 }];

    // CreateTriangleMeshShape()
    let object_to_world: Transform = Transform::translate(Vector3f {
        x: 0.25,
        y: 0.0,
        z: 0.0,
    });
    let world_to_object: Transform = Transform::inverse(object_to_world);
    // reverseOrientation = false
    // p graphicsState.floatTextures
    // $7 = std::map with 0 elements

    // transform mesh vertices to world space
    let mut p_ws: Vec<Point3f> = Vec::new();
    let n_vertices: usize = p.len();
    for i in 0..n_vertices {
        p_ws.push(object_to_world.transform_point(p[i]));
    }
    let s: Vec<Vector3f> = Vec::new();
    let n: Vec<Vector3f> = Vec::new();
    let mut builder: SceneDescriptionBuilder = SceneDescriptionBuilder::new();
    println!("########");
    println!("# mesh #");
    println!("########");
    builder.add_mesh(object_to_world,
                     world_to_object,
                     n_triangles,
                     vertex_indices,
                     n_vertices,
                     p_ws, // in world space
                     s, // empty
                     n, // empty
                     uv);

    // sphere

    // Translate -1.3 0 0
    let object_to_world: Transform = Transform::translate(Vector3f {
        x: -1.3,
        y: 0.0,
        z: 0.0,
    });
    let world_to_object: Transform = Transform::inverse(object_to_world);

    // Shape "sphere"
    let radius: Float = 1.0;
    let z_min: Float = -1.0;
    let z_max: Float = 1.0;
    let phi_max: Float = 360.0;
    println!("############");
    println!("# sphere 1 #");
    println!("############");
    builder.add_sphere(object_to_world,
                       world_to_object,
                       radius,
                       z_min,
                       z_max,
                       phi_max);

    // sphere

    // Translate 2.6 0 0 (not protected by Attribute[Begin|End])
    let object_to_world: Transform = object_to_world *
                                     Transform::translate(Vector3f {
        x: 2.6,
        y: 0.0,
        z: 0.0,
    });
    let world_to_object: Transform = Transform::inverse(object_to_world);

    // Shape "sphere"
    println!("############");
    println!("# sphere 2 #");
    println!("############");
    builder.add_sphere(object_to_world,
                       world_to_object,
                       radius,
                       z_min,
                       z_max,
                       phi_max);

    let scene_description: SceneDescription = builder.finalize();

    // pbrtWorldEnd

    // RenderOptions::MakeIntegrator
    let xw: Float = 0.5;
    let yw: Float = 0.5;
    let box_filter = BoxFilter {
        radius: Vector2f { x: xw, y: yw },
        inv_radius: Vector2f {
            x: 1.0 / xw,
            y: 1.0 / yw,
        },
    };
    // (gdb) p *film->filter
    // radius = {x = 0.5, y = 0.5}
    // invRadius = {x = 2, y = 2}
    let filename: String = String::from("spheres-differentials-texfilt.exr");
    let xres = 1000;
    let yres = 500;
    let crop: Bounds2f = Bounds2f {
        p_min: Point2f { x: 0.0, y: 0.0 },
        p_max: Point2f { x: 1.0, y: 1.0 },
    };
    let film: Film = Film::new(Point2i { x: xres, y: yres },
                               crop,
                               box_filter,
                               35.0,
                               filename,
                               1.0,
                               std::f64::INFINITY);
    // TODO: pixels (see gdb output below)
    // (gdb) p *film
    // fullResolution = {x = 1000, y = 500},
    // diagonal = 0.0350000001
    // filter = std::unique_ptr<pbrt::Filter> containing 0x10a6080
    // filename = "spheres-differentials-texfilt.exr"
    // croppedPixelBounds = {pMin = {x = 0, y = 0}, pMax = {x = 1000, y = 500}}
    // pixels = std::unique_ptr<pbrt::Film::Pixel> ...
    // scale = 1
    // maxSampleLuminance = inf
    println!("########");
    println!("# film #");
    println!("########");
    println!("film = {:?}", film);
    // pbrt::MakeCamera
    let pos = Point3f {
        x: 2.0,
        y: 2.0,
        z: 5.0,
    };
    let look = Point3f {
        x: 0.0,
        y: -0.4,
        z: 0.0,
    };
    let up = Vector3f {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    let t: Transform = Transform::look_at(pos, look, up);
    let it: Transform = Transform {
        m: t.m_inv.clone(),
        m_inv: t.m.clone(),
    };
    let animated_cam_to_world: AnimatedTransform = AnimatedTransform::new(&it, 0.0, &it, 1.0);
    // pbrt::CreatePerspectiveCamera
    let shutteropen: Float = 0.0;
    let shutterclose: Float = 1.0;
    let lensradius: Float = 0.0;
    let focaldistance: Float = 1e6;
    let frame: Float = xres as Float / yres as Float;
    let mut screen: Bounds2f = Bounds2f::default();
    if frame > 1.0 {
        screen.p_min.x = -frame;
        screen.p_max.x = frame;
        screen.p_min.y = -1.0;
        screen.p_max.y = 1.0;
    } else {
        screen.p_min.x = -1.0;
        screen.p_max.x = 1.0;
        screen.p_min.y = -1.0 / frame;
        screen.p_max.y = 1.0 / frame;
    }
    let fov: Float = 30.0;
    let camera_to_screen: Transform = Transform::perspective(fov, 1e-2, 1000.0);
    let perspective_camera: PerspectiveCamera = PerspectiveCamera::new(animated_cam_to_world,
                                                                       camera_to_screen,
                                                                       screen,
                                                                       shutteropen,
                                                                       shutterclose,
                                                                       lensradius,
                                                                       focaldistance,
                                                                       fov,
                                                                       film /* ,
                                                                             * medium */);
    println!("##########");
    println!("# camera #");
    println!("##########");
    println!("perspective_camera = {:?}", perspective_camera);
    // pbrt::MakeSampler
    let sampler: ZeroTwoSequenceSampler = ZeroTwoSequenceSampler {
        samples_per_pixel: 1,
        n_sampled_dimensions: 4,
    };
    let pixel_bounds: Bounds2i = Bounds2i {
        p_min: Point2i { x: 0, y: 0 },
        p_max: Point2i { x: xres, y: yres },
    };
    let integrator: DirectLightingIntegrator =
        DirectLightingIntegrator::new(LightStrategy::UniformSampleAll,
                                      10,
                                      perspective_camera,
                                      sampler,
                                      pixel_bounds);
    println!("##############");
    println!("# integrator #");
    println!("##############");
    println!("integrator = {:?}", integrator);
    // TMP: process SceneDescription before handing primitives to BVHAccel
    let scene: Scene = Scene::new(&scene_description);
    // TMP: process SceneDescription before handing primitives to BVHAccel
    // pbrt::RenderOptions::MakeScene
    let accelerator: BVHAccel = BVHAccel::new(scene.primitives, 4, SplitMethod::SAH);
}