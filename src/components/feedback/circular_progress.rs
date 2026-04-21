#[derive(Debug, Clone)]
pub struct CircularProgress {
    size:     CircularProgressSize,
    progress: f64
}

#[derive(Debug, Clone, Default)]
pub enum CircularProgressSize {
    #[default]
    Medium,
    Small,
    Large
}

impl CircularProgress {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            size:     CircularProgressSize::Medium,
            progress: 0.0
        }
    }

    #[must_use]
    pub fn size(mut self, size: &str) -> Self {
        self.size = match size {
            "small" => CircularProgressSize::Small,
            "large" => CircularProgressSize::Large,
            _ => CircularProgressSize::Medium
        };
        self
    }

    #[must_use]
    pub const fn progress(mut self, progress: f64) -> Self {
        self.progress = progress.clamp(0.0, 100.0);
        self
    }

    #[must_use]
    pub fn render(&self) -> String {
        let (size, stroke_width) = match self.size {
            CircularProgressSize::Small => (24.0, 3.0),
            CircularProgressSize::Medium => (32.0, 4.0),
            CircularProgressSize::Large => (48.0, 6.0)
        };

        let radius = (size - stroke_width) / 2.0;
        let circumference = 2.0 * std::f64::consts::PI * radius;
        let dash_offset = circumference * ((100.0 - self.progress) / 100.0);

        format!(
            r#"<svg class="telegram-ui-circular-progress" width="{}" height="{}" fill="none" xmlns="http://www.w3.org/2000/svg">
  <circle cx="{}" cy="{}" r="{}" stroke="var(--tgui-hint, #c4c4c4)" stroke-opacity="0.1" stroke-width="{}" fill="none"/>
  <circle cx="{}" cy="{}" r="{}" fill="none" stroke="var(--tgui-link, #2481cc)" stroke-width="{}" stroke-linecap="round" stroke-dasharray="{}" stroke-dashoffset="{}"/>
</svg>"#,
            size as i32,
            size as i32,
            size / 2.0,
            size / 2.0,
            radius,
            stroke_width,
            size / 2.0,
            size / 2.0,
            radius,
            stroke_width,
            circumference as i32,
            dash_offset as i32
        )
    }
}

impl Default for CircularProgress {
    fn default() -> Self {
        Self::new()
    }
}
