use crate::vertex::Vertex;

#[derive(Debug, Clone)]
pub struct PolyLine {
    pub vertices: Vec<Vertex>,
    pub closed: bool
}

impl PolyLine {
    pub fn from_sections(first: &PolyLine, second: &PolyLine) -> PolyLine {
        let mut vertices = first.vertices.clone();
        vertices.pop();
        vertices.append(second.vertices.clone().as_mut());

        PolyLine {vertices, closed: false}
    }

    pub fn continues_from(&self, previous: &PolyLine) -> bool {
        if let (Some(a), Some(b)) = (self.vertices.first(), previous.vertices.last()) {
            return a.distance_to(b) < 0.001;
        }

        false
    }
}

pub fn glue_polylines(lines: Vec<PolyLine>) -> Vec<PolyLine> {
    let mut new_lines = vec![];

    let mut iterator = lines.iter();
    let mut last_line = None;
    let mut tmp: PolyLine;

    while let Some(line) = iterator.next() {
        match (last_line, line) {
            (Some(l), r) if r.continues_from(l) => {
                tmp = PolyLine::from_sections(l, r);
                last_line = Some(&tmp);
            },

            (None, _) => {
                last_line = Some(line);
            },

            _ => {
                new_lines.push(last_line.unwrap().clone());
                last_line = Some(line);
            }
        }
    }

    if let Some(ll) = last_line {
        new_lines.push(ll.clone());
    }

    new_lines
}
