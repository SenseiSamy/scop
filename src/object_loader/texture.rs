use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl Texture {
    pub fn parse_ppm(file: &str) -> Result<Self> {
        let lines: Vec<&str> = file
            .lines()
            .filter(|l| !l.starts_with("#") && !l.is_empty())
            .collect();

        if lines.len() < 3 || lines[0] != "P3" {
            return Err(anyhow!("Invalid ppm file"));
        }

        let size: Vec<&str> = lines[1].split(' ').collect();
        let mut texture = Texture {
            width: size[0].parse()?,
            height: size[1].parse()?,
            data: Vec::new(),
        };

        let max_value: u16 = lines[2].parse()?;

        let mut i = 0;
        for line in lines.iter().skip(3) {
            for value in line.split_ascii_whitespace() {
                let value = (value.parse::<f64>()? / max_value as f64) * 255.0;
                texture.data.push(value as u8);
                if i == 2 {
                    texture.data.push(255);
                }
                i = (i + 1) % 3;
            }
        }

        if texture.data.len() as u32 != 4 * texture.height * texture.width {
            return Err(anyhow!(
                "The ppm file doesnt contain all the rgb values for its dimensions"
            ));
        }

        Ok(texture)
    }
}
