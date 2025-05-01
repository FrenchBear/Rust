// Raytracer in Rust
// from https://joshondesign.com/2014/09/17/rustlang
// 2018-10-23	PV	    Significant updates to make original code compilable (language has changed??)

struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector {
    fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x: x, y: y, z: z }
    }
    fn scale(&self, s: f32) -> Vector {
        Vector {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }
    fn plus(&self, b: &Vector) -> Vector {
        Vector::new(self.x + b.x, self.y + b.y, self.z + b.z)
    }
    fn minus(&self, b: &Vector) -> Vector {
        Vector::new(self.x - b.x, self.y - b.y, self.z - b.z)
    }
    fn dot(&self, b: &Vector) -> f32 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }
    fn magnitude(&self) -> f32 {
        (self.dot(self)).sqrt()
    }
    fn normalize(&self) -> Vector {
        self.scale(1.0 / self.magnitude())
    }
}

struct Ray {
    orig: Vector,
    dir: Vector,
}

#[derive(Copy, Clone)]
struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    fn scale(&self, s: f32) -> Color {
        Color {
            r: self.r * s,
            g: self.g * s,
            b: self.b * s,
        }
    }
    fn plus(&self, b: Color) -> Color {
        Color {
            r: self.r + b.r,
            g: self.g + b.g,
            b: self.b + b.b,
        }
    }
}

struct Sphere {
    center: Vector,
    radius: f32,
    color: Color,
}

impl Sphere {
    fn get_normal(&self, pt: &Vector) -> Vector {
        return pt.minus(&self.center).normalize();
    }
}

struct Light {
    position: Vector,
    color: Color,
}

// few constants.
static WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, };
static RED: Color = Color { r: 1.0, g: 0.0, b: 0.0, };
static GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0, };
static BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0, };

static LIGHT1: Light = Light {
    position: Vector {
        x: 0.7,
        y: -1.0,
        z: 1.7,
    },
    color: WHITE,
};

// in my main function I'll set up the scene and create a lookup table of one letter strings for text mode rendering.

fn main() {
    let lut = vec![".", "-", "+", "X", "#", "@"];

    let w = 20 * 4;
    let h = 10 * 4;

    let scene = vec![
        Sphere {
            center: Vector::new(-1.0, 0.0, 3.0),
            radius: 0.3,
            color: RED,
        },
        Sphere {
            center: Vector::new(0.0, 0.0, 3.0),
            radius: 0.8,
            color: GREEN,
        },
        Sphere {
            center: Vector::new(1.0, 0.0, 3.0),
            radius: 0.3,
            color: BLUE,
        },
    ];

    // Now lets get to the core ray tracing loop. This looks at every pixel to see if it's ray intersects with the spheres
    // in the scene. It should be mostly understandable, but you'll start to see the differences with C.

    for j in 0..h {
        println!("--");
        for i in 0..w {
            //let tMax = 10000f32;
            let fw: f32 = w as f32;
            let fi: f32 = i as f32;
            let fj: f32 = j as f32;
            let fh: f32 = h as f32;

            let ray = Ray {
                orig: Vector::new(0.0, 0.0, 0.0),
                dir: Vector::new((fi - fw / 2.0) / fw, (fj - fh / 2.0) / fh, 1.0).normalize(),
            };

            let mut obj_hit_obj: Option<(&Sphere, f32)> = None;

            for obj in scene.iter() {
                let ret = intersect_sphere(&ray, &(obj.center), obj.radius);
                if ret.hit {
                    obj_hit_obj = Some((obj, ret.tval));
                }
            }

            // The for loops are done with a range function which returns an iterator. Iterators are used extensively in Rust
            // because they are inherently safer than direct indexing.

            // Notice the obj_hit_obj variable. It is set based on the result of the intersection test. In JavaScript I used several
            // variables to track if an object had been hit, and to hold the hit object and hit distance if it did intersect. In
            // Rust you are encouraged to use options. An Option is a special enum with two possible values: None and Some. If it is
            // None then there is nothing inside the option. If it is Some then you can safely grab the contained object. Options
            // are a safer alternative to null pointer checks.

            // Options can hold any object thanks to Rust's generics. In the code above I tried out something tricky and
            // surprisingly it worked. Since I need to store several values I created an option holding a tuple, which is like a
            // fixed size array with fixed types. obj_hit_obj is defined as an option holding a tuple of a Sphere and an f32 value.
            // When I check if ret.hit is true I set the option to Some((*obj,ret.tval)), meaning the contents of my object pointer
            // and the hit distance.

            // Now lets look at the second part of the loop, once ray intersection is done.

            let pixel = match obj_hit_obj {
                Some((obj, tval)) => lut[shade_pixel(&ray, &obj, tval)],
                None => " ",
            };

            print!("{}", pixel);
        }
    }
}

// Finally I can check and retrieve the option values using an if statement or a match. Match is like a switch or case
// statement in C, but with super powers. It forces you to account for all possible code paths. This ensures there are
// no mistakes during compilation. In the code above I match the some and none cases. In the Some case it pulls out the
// nested objects and gives them the names obj and tval, just like the tuple I stuffed into it earlier. This is called
// destructuring in Rust. If there is a value then it calls shade_pixel and returns character in the look up table
// representing that grayscale value. If the None case happens then it returns a space. In either case we know the pixel
// variable will have a valid value after the match. It's impossible for pixel to be null, so I can safely print it.

// The rest of my code is basically vector math. It looks almost identical to the same code in JavaScript, just strongly
// typed.

fn shade_pixel(ray: &Ray, obj: &Sphere, tval: f32) -> usize {
    let pi = ray.orig.plus(&ray.dir.scale(tval));
    let color = diffuse_shading(&pi, obj, &LIGHT1);
    let col = (color.r + color.g + color.b) / 3.0;
    (col * 6.0) as usize
}

struct HitPoint {
    hit: bool,
    tval: f32,
}

fn intersect_sphere(ray: &Ray, center: &Vector, radius: f32) -> HitPoint {
    let l = center.minus(&ray.orig);
    let tca = l.dot(&ray.dir);
    if tca < 0.0 {
        return HitPoint {
            hit: false,
            tval: -1.0,
        };
    }
    let d2 = l.dot(&l) - tca * tca;
    let r2 = radius * radius;
    if d2 > r2 {
        return HitPoint {
            hit: false,
            tval: -1.0,
        };
    }
    let thc = (r2 - d2).sqrt();
    let t0 = tca - thc;
    if t0 > 10000.0 {
        return HitPoint {
            hit: false,
            tval: -1.0,
        };
    }
    return HitPoint {
        hit: true,
        tval: t0,
    };
}

fn clamp(x: f32, a: f32, b: f32) -> f32 {
    if x < a {
        return a;
    }
    if x > b {
        return b;
    }
    return x;
}

fn diffuse_shading(pi: &Vector, obj: &Sphere, light: &Light) -> Color {
    let n = obj.get_normal(pi);
    let lam1 = light.position.minus(pi).normalize().dot(&n);
    let lam2 = clamp(lam1, 0.0, 1.0);
    light.color.scale(lam2 * 0.5).plus(obj.color.scale(0.3))
}
