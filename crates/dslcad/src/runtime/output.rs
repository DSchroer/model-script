use super::Value;
use opencascade::{Error, Point, Shape, Wire};
use std::cell::{Ref, RefMut};
use threemf::protocol::Part;

pub trait IntoPart {
    fn into_part(self) -> Result<Part, Error>;
}

impl IntoPart for &f64 {
    fn into_part(self) -> Result<Part, Error> {
        Ok(Part::Data {
            text: self.to_string(),
        })
    }
}

impl IntoPart for &bool {
    fn into_part(self) -> Result<Part, Error> {
        Ok(Part::Data {
            text: self.to_string(),
        })
    }
}

impl IntoPart for &str {
    fn into_part(self) -> Result<Part, Error> {
        Ok(Part::Data {
            text: self.to_string(),
        })
    }
}

impl IntoPart for &Vec<Value> {
    fn into_part(self) -> Result<Part, Error> {
        Ok(Part::Data {
            text: format!("[{}]", self.len()),
        })
    }
}

impl IntoPart for Ref<'_, Point> {
    fn into_part(self) -> Result<Part, Error> {
        Ok(Part::Planar {
            points: vec![[self.x(), self.y(), self.z()]],
            lines: Vec::new(),
        })
    }
}

impl IntoPart for RefMut<'_, Wire> {
    fn into_part(mut self) -> Result<Part, Error> {
        Ok(Part::Planar {
            points: [self.start()?, self.end()?]
                .into_iter()
                .flatten()
                .map(|p| p.into())
                .collect(),
            lines: self.points()?,
        })
    }
}

impl IntoPart for RefMut<'_, Shape> {
    fn into_part(mut self) -> Result<Part, Error> {
        let original = self.mesh()?;
        let mut mesh = threemf::protocol::Mesh {
            vertices: original.vertices,
            triangles: vec![],
            normals: vec![],
        };

        for (tri, normal) in self.mesh()?.triangles_with_normals() {
            mesh.triangles.push(*tri);
            mesh.normals.push(normal);
        }

        Ok(Part::Object {
            points: self.points()?,
            lines: self.lines()?,
            mesh,
        })
    }
}
