pub mod segment;
pub use segment::*;


use std::fmt::Display;

#[derive(Clone, PartialEq, Debug)]
pub struct Path
{
    pub segments: Vec<String>
}

impl Path {
    pub fn join(&self, segment: impl Into<String>) -> Path
    {
        let mut clone = self.clone();
        clone.segments.push(segment.into());
        clone
    }
}

impl Default for Path {
    fn default() -> Self {
        let segments = Vec::new();
        Self { segments }
    }
}

impl<'a> From<&'a str> for Path {
    fn from(value: &'a str) -> Self {
        let segments = value.split("::").map(String::from).collect();
        Self { segments }
    }
}

impl From<Vec<&str>> for Path
{
    fn from(value: Vec<&str>) -> Path {
        let segments = value.iter().map(|s| s.to_string()).collect();
        Path { segments }
    }
}

impl From<Vec<String>> for Path
{
    fn from(value: Vec<String>) -> Path {
        let segments = value;
        Path { segments }
    }
}

impl From<&[&str]> for Path
{
    fn from(value: &[&str]) -> Path {
        let segments = value.iter().map(|s| s.to_string()).collect();
        Path { segments }
    }
}

impl From<&[String]> for Path
{
    fn from(value: &[String]) -> Path {
        let segments = value.to_vec();
        Path { segments }
    }
}

impl Display for Path {
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
        let path = Path::from(slice);
        assert_eq!(path.segments, ["A", "B", "C"]);
    }

    #[test]
    fn from_vector() {
        let vector = vec!["A", "B", "C"];
        let path = Path::from(vector);
        assert_eq!(path.segments, ["A", "B", "C"]);
    }
}
