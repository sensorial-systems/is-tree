pub mod segment;
pub use segment::*;


use std::fmt::Display;

#[derive(Clone, PartialEq, Debug)]
pub struct Path<Segment>
{
    pub segments: Vec<Segment>
}

impl<Segment> Path<Segment> {
    pub fn join(&self, segment: impl Into<Segment>) -> Path<Segment>
    where Path<Segment>: Clone
    {
        let mut clone = self.clone();
        clone.segments.push(segment.into());
        clone
    }
}

impl<Segment> Default for Path<Segment> {
    fn default() -> Self {
        let segments = Vec::new();
        Self { segments }
    }
}

impl<'a> From<&'a str> for Path<&'a str> {
    fn from(value: &'a str) -> Self {
        let segments = value.split("::").collect();
        Self { segments }
    }
}

impl<Segment> From<Vec<Segment>> for Path<Segment>
{
    fn from(value: Vec<Segment>) -> Path<Segment> {
        let segments = value;
        Path { segments }
    }
}

impl<'a, Segment> From<&'a [Segment]> for Path<Segment>
where Segment: Copy
{
    fn from(value: &'a [Segment]) -> Path<Segment> {
        let segments = value.to_vec();
        Path { segments }
    }
}

impl Display for Path<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.segments.join("::"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_string() {
        let path = Path::from("A");
        assert_eq!(path.segments, ["A"]);
        let path = Path::from("A::B");
        assert_eq!(path.segments, ["A", "B"]);
    }

    #[test]
    fn from_array() {
        let array = ["A", "B", "C"];
        let slice = array.as_slice();
        let path: Path<_> = Path::from(slice);
        assert_eq!(path.segments, ["A", "B", "C"]);
    }

    #[test]
    fn from_vector() {
        let vector = vec!["A", "B", "C"];
        let path = Path::from(vector);
        assert_eq!(path.segments, ["A", "B", "C"]);
    }
}
