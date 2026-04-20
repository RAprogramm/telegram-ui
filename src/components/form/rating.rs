#[derive(Debug, Clone)]
pub struct Rating {
    max_stars: usize,
    value:     f64,
    size:      String
}

impl Rating {
    pub fn new() -> Self {
        Self {
            max_stars: 5,
            value:     0.0,
            size:      "m".to_string()
        }
    }

    pub fn max_stars(mut self, max: usize) -> Self {
        self.max_stars = max;
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = value;
        self
    }

    pub fn size(mut self, size: &str) -> Self {
        self.size = size.to_string();
        self
    }

    pub fn render(&self) -> String {
        let stars: String = (0..self.max_stars)
            .map(|i| {
                let filled = if (self.value - i as f64) >= 0.5 {
                    "rating-star--filled"
                } else {
                    ""
                };
                format!("<span class=\"rating-star {}\">⭐</span>", filled)
            })
            .collect();

        format!(
            "<div class=\"telegram-ui-rating rating--{}\">{}</div>",
            self.size, stars
        )
    }
}

impl Default for Rating {
    fn default() -> Self {
        Self::new()
    }
}
