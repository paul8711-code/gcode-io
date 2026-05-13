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

/// An x, y and z-value in a 3D space
pub type Point3D = [f64; 3];

/// A triangle consisting of the indexes of all points and a normal vector
pub struct Triangle {
    /// The 3 indexes for every triangle corner
    pub vertices: [usize; 3],
    /// Normal vector of the triangle. Does not have to be normalized.
    pub normal: Point3D
}

/// Mesh consisting of [vertices](type.Point3D.html) and [triangles](struct.Triangle.html)
pub struct Mesh {
    /// A vector of all the points in the mesh
    pub vertices: Vec<Point3D>,
    /// A vector of [triangles](struct.Triangle.html)
    pub faces: Vec<Triangle>
}

/// Parses a Mesh into G-code instructions.
///
/// This function parses the given Mesh and writes the G-code commands directly into the given
/// output stream.
pub fn parse(writer: &mut impl std::io::Write, mesh: Mesh) -> std::io::Result<()> {
    // mesh
    // slice at z heights (layer height offset) -> check if each edge crosses z-plane
    // connect line segments
    // generate toolpaths (path that the nozzle (3d printer) takes)
    // create gcode based on above
    Ok(())
}
