#[derive(Debug, Clone)]
pub struct Slider {
    min: f64,
    max: f64,
    value: f64,
    step: f64,
    disabled: bool,
    ios: bool,
}

impl Slider {
    pub fn new() -> Self {
        Self {
            min: 0.0,
            max: 100.0,
            value: 50.0,
            step: 1.0,
            disabled: false,
            ios: false,
        }
    }

    pub fn min(mut self, min: f64) -> Self {
        self.min = min;
        self
    }

    pub fn max(mut self, max: f64) -> Self {
        self.max = max;
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = value.clamp(self.min, self.max);
        self
    }

    pub fn step(mut self, step: f64) -> Self {
        self.step = step;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn ios(mut self, ios: bool) -> Self {
        self.ios = ios;
        self
    }

    pub fn render(&self) -> String {
        let ios_class = if self.ios { "slider--ios" } else { "" };
        let disabled_class = if self.disabled {
            "slider--disabled"
        } else {
            ""
        };

        let percentage = ((self.value - self.min) / (self.max - self.min) * 100.0) as i32;

        format!(
            r#"<div class="telegram-ui-slider {} {}">
  <input type="range" class="slider-input" min="{}" max="{}" value="{}" step="{}" {} />
  <div class="slider-track">
    <div class="slider-track-filled" style="width: {}%"></div>
  </div>
</div>"#,
            ios_class,
            disabled_class,
            self.min,
            self.max,
            self.value,
            self.step,
            if self.disabled { "disabled" } else { "" },
            percentage
        )
    }
}
