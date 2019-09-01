use crate::arc::Arc;
use crate::vertex::Vertex;
use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct VertexWithBulge {
    pub vertex: Vertex,
    pub bulge: f64
}

fn get_bulge_arc(start_point: &Vertex, end_point: &Vertex, bulge: f64) -> Arc {
    let bulge_sign = 1f64.copysign(bulge);
    let chord_length = start_point.distance_to(end_point);

    let sagitta = (chord_length / 2.0 * bulge).abs();
    let radius = ((sagitta / 2.0) + (chord_length.powf(2.0) / (8.0 * sagitta))).abs();
    let apothem = radius - sagitta;

    let midpoint = start_point.midpoint(end_point);

    let centre_vector = start_point.vector_to(end_point).normal().with_magnitude(apothem * -bulge_sign);

    let centre = midpoint + centre_vector;

    let start_angle = (start_point.y - centre.y).atan2(start_point.x - centre.x);
    let mut end_angle = (end_point.y - centre.y).atan2(end_point.x - centre.x);

    if end_angle < start_angle && bulge_sign.is_sign_positive() {
        end_angle += 2.0 * PI;
    }

    if end_angle > start_angle && bulge_sign.is_sign_negative() {
        end_angle -= 2.0 * PI;
    }

    Arc {
        centre,
        radius,
        start_angle,
        end_angle
    }
}


pub fn explode_bulged_vertices(vertices: Vec<VertexWithBulge>) -> Vec<Vertex> {
    let mut new_vertices = vec![];

    let mut iterator = vertices.iter();
    let mut last_vertex: Option<&VertexWithBulge> = None;

    while let Some(vertex) = iterator.next() {
        match (last_vertex, vertex) {
            (Some(l), r) if l.bulge != 0.0 => {
                let a = get_bulge_arc(&l.vertex, &r.vertex, l.bulge);
                new_vertices.push(l.vertex.clone());
                new_vertices.append(&mut a.into_polyline().vertices);
                last_vertex = Some(vertex);
            },

            (None, _) => {
                last_vertex = Some(vertex);
            },

            _ => {
                new_vertices.push(last_vertex.unwrap().vertex.clone());
                last_vertex = Some(vertex);
            }
        }
    }

    if let Some(ll) = last_vertex {
        new_vertices.push(ll.vertex.clone());

        if ll.bulge != 0.0 {
            let first = vertices.first().unwrap();
            let a = get_bulge_arc(&ll.vertex, &first.vertex, ll.bulge);
            new_vertices.append(&mut a.into_polyline().vertices);
        }
    }

    new_vertices
}
