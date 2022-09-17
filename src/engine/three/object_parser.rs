peg::parser!(
    grammar obj_parser() for str {
        pub rule document() -> Vec<Line>
            = (line() ** "\n")

        rule line() -> Line
            = vertex()
            / face()
            / any() {Line::Other}

        rule any() -> Vec<char>
            = [' '..]*

        rule vertex() -> Line
            = "v " x:float() " " y:float() " " z:float() " " w:float() { Line::Vertex(x, y, z, Some(w)) }
            / "v " x:float() " " y:float() " " z:float()               { Line::Vertex(x, y, z, None) }

        rule vertex_texture() -> Line
            = "vt " x:float() " " y:float() { Line::VertexTexture(x, y) }

        rule vertex_normal() -> Line
            = "vn " x:float() " " y:float() " " z:float() " " w:float() { Line::VertexNormal(x, y, z, Some(w)) }
            / "vn " x:float() " " y:float() " " z:float()               { Line::VertexNormal(x, y, z, None) }

        rule float() -> f64
            = s:$("-"? ['0'..='9']+ ("." ['0'..='9']+)?) { s.parse::<f64>().unwrap() }

        rule int() -> usize
            = s:$(['0'..='9']+) { s.parse::<usize>().unwrap() }

        rule face() -> Line
            = "f " indices:(indices() ** " ") { Line::Face(indices) }

        rule indices() -> (usize, Option<usize>, Option<usize>)
            = v:int() "/" vt:int() "/" vn:int() { (v, Some(vt), Some(vn)) }
            / v:int() "//" vn:int()             { (v, None, Some(vn)) }
            / v:int() "/" vt:int()              { (v, Some(vt), None) }
            / v:int()                           { (v, None, None) }
    }
);

pub use obj_parser::*;

pub enum Line {
    Vertex(f64, f64, f64, Option<f64>),
    VertexTexture(f64, f64),
    VertexNormal(f64, f64, f64, Option<f64>),
    Face(Vec<(usize, Option<usize>, Option<usize>)>),
    Other,
}
