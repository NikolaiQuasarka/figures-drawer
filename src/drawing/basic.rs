#[derive(PartialEq, Debug)]
pub struct Size(pub u32, pub u32);

#[derive(Debug, Clone)]
pub struct Point(pub i32, pub i32);

impl std::str::FromStr for Size {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [width, height, ..] = s.trim().split_whitespace().collect::<Vec<_>>()[..] else {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Parsing Error",
            )));
        };

        let width = width.parse()?;
        let height = height.parse()?;

        Ok(Self(width, height))
    }
}

impl Point {
    pub fn default() -> Point {
        Point(0, 0)
    }
}

impl std::str::FromStr for Point {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.trim();
        if parts.is_empty() {
            return Ok(Self::default());
        }

        let [x, y, ..] = splits.split_whitespace().collect::<Vec<_>>()[..] else {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Parsing Error",
            )));
        };

        let x = x.parse()?;
        let y = y.parse()?;

        Ok(Self(x, y))
    }
}
