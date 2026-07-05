/// A coordinate representing a location in 3D space.
///
/// Stored as an array of `[x, y, z]`.
pub type Point3D = [f64; 3];

/// A face of a 3D mesh defined by vertex indexing and a surface normal.
pub struct Triangle {
    /// The indices of the three vertices that form this triangle.
    /// These usually point to elements in a [`Vec<Point3D>`].
    pub indices: [usize; 3],
    /// The surface normal vector of the triangle.
    ///
    /// Note: This vector indicates direction but is not guaranteed to be
    /// a unit vector (length of 1.0).
    pub normal: Point3D,
}

/// A 3D model represented as a collection of vertices and triangular faces.
///
/// The `Mesh` uses indexed geometry to save memory, where multiple [`Triangle`]
/// faces can share the same [`Point3D`] vertex.
pub struct Mesh {
    /// A vector containing all points of the mesh
    pub vertices: Vec<Point3D>,
    /// A vector containing all [`Triangle`] faces of the [`Mesh`]
    pub faces: Vec<Triangle>,
}

/// A builder for constructing a [`Mesh`].
///
/// The `MeshBuilder` allows you to incrementally add vertices and define triangular faces by
/// referencing those vertices by their index.
///
/// # Example
///
/// ```
/// use gcode_io::mesh::MeshBuilder;
/// let mesh = MeshBuilder::new()
///     .add_vertex(0.0, 0.0, 0.0)
///     .add_vertex(1.0, 0.0, 0.0)
///     .add_vertex(0.0, 1.0, 0.0)
///     .add_triangle(0, 1, 2)
///     .build();
/// ```
pub struct MeshBuilder {
    vertices: Vec<Point3D>,
    indices: Vec<[usize; 3]>,
}

impl Default for MeshBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl MeshBuilder {
    /// Creates a new, empty `MeshBuilder`
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    /// Adds a single vertex to the mesh and returns the builder.
    ///
    /// The vertex is assigned an index based on the order it was added,
    /// starting at 0.
    pub fn add_vertex(mut self, x: f64, y: f64, z: f64) -> Self {
        self.vertices.push([x, y, z]);
        self
    }

    /// Adds a single triangle to the mesh using vertex indices.
    ///
    /// # Arguments
    ///
    /// * `a`, `b`, `c` - The indices of the vertices (added via [`add_vertex`](MeshBuilder::add_vertex))
    ///   that form the corners of the triangle.
    pub fn add_triangle(mut self, a: usize, b: usize, c: usize) -> Self {
        self.indices.push([a, b, c]);
        self
    }

    /// Consumes the builder and returns a completed [`Mesh`].
    ///
    /// This method calculates face normals based on the provided vertices
    /// and indices.
    ///
    /// # Panics
    ///
    /// Panics if any of the indices provided in [`add_triangle`](MeshBuilder::add_triangle) are out of
    /// bounds of the vertex list.
    pub fn build(self) -> Mesh {
        let mut faces = Vec::new();

        for [i_a, i_b, i_c] in self.indices {
            let a = self.vertices[i_a];
            let b = self.vertices[i_b];
            let c = self.vertices[i_c];

            // U = (b - a)
            // V = (c - a)
            // Nx = Uy * Vz - Uz * Vy
            // Ny = Uz * Vx - Ux * Vz
            // Nz = Ux * Vy - Uy * Vx

            let normal = Self::cross_product(Self::subtract(b, a), Self::subtract(c, a));

            faces.push(Triangle {
                indices: [i_a, i_b, i_c],
                normal,
            });
        }

        Mesh {
            vertices: self.vertices,
            faces,
        }
    }

    fn subtract(b: [f64; 3], a: [f64; 3]) -> [f64; 3] {
        [b[0] - a[0], b[1] - a[1], b[2] - a[2]]
    }

    fn cross_product(u: [f64; 3], v: [f64; 3]) -> [f64; 3] {
        [
            u[1] * v[2] - u[2] * v[1],
            u[2] * v[0] - u[0] * v[2],
            u[0] * v[1] - u[1] - v[0],
        ]
    }
}
