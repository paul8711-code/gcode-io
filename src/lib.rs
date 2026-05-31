mod math;
pub mod mesh;
use mesh::{Mesh, Point3D};

// new datatype: Mesh: consists of Vec<Triangle> and Vec<Point3D>
// Triangle: normal, vertices (3) -> [usize; 3] (usize bcs its the index in the Vec<Point3D>)
// normal: Normal, is the vector thats in a 90° angle to the triangle
// if normal.x is positive, air is in the direction where x gets bigger
// that means that the infill has to be in the other direction
// if normal.y is negative, air is in the direction where y gets smaller
// that means that the infill has to be in the other direction
// same for z

// what to do about the toolpaths? once we get to it, i will make some kind of modular system
// allowing users to implement their own "tools" (customize infill, patterns, layer height, ...)

// system similar to this:
// pub struct SlicerConfig {}
// pub trait SliceProfile {}

/// Parses a Mesh into G-code instructions.
///
/// This function parses the given Mesh and writes the G-code commands directly into the given
/// output stream.
pub fn parse(writer: &mut impl std::io::Write, mesh: Mesh) -> std::io::Result<()> {
    // mesh
    // slice at z heights (layer height offset) -> check if each edge crosses z-plane:
    // get point on line that has z height by layer height bigger
    // connect line segments
    // generate toolpaths (path that the nozzle (3d printer) takes)
    // infill based on the inverse normal vectors (finally understand it)
    // create gcode based on above
    Ok(())
}
