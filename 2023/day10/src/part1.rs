pub fn run(input: &str) -> usize {
    let sketch = crate::common::parse(input).unwrap();
    sketch.get_loop().len() / 2
}
