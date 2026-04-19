#[derive(PartialEq, Debug)]
pub struct Size(pub u32, pub u32);

#[derive(Debug, Clone)]
pub struct Point(pub i32, pub i32);

impl Point {
    pub fn default() -> Point {
        Point(0, 0)
    }
}

impl std::str::FromStr for Point {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .trim()
            .split_once(',')
            .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::InvalidInput))?;

        let x = x.parse()?;
        let y = y.parse()?;

        Ok(Self(x, y))
    }
}
