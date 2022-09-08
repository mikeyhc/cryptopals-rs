pub trait Xor<T> {
    fn xor(&self, other: &T) -> Self;
}

pub trait Score {
    fn score(&self) -> f32;
}
