// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Steps component for Telegram UI

use std::fmt;

/// Step state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StepState {
    #[default]
    /// Step is pending
    Pending,
    /// Step is in progress
    InProgress,
    /// Step is completed
    Completed,
    /// Step failed
    Failed
}

/// Individual step
#[derive(Debug, Clone)]
pub struct Step {
    number:   usize,
    title:    String,
    subtitle: Option<String>,
    #[expect(dead_code)]
    state:    StepState
}

/// Steps component - a vertical or horizontal step indicator
#[derive(Debug, Clone)]
pub struct Steps {
    steps:       Vec<Step>,
    current:     usize,
    orientation: Orientation
}

/// Steps orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Orientation {
    #[default]
    /// Vertical orientation
    Vertical,
    /// Horizontal orientation
    Horizontal
}

impl Steps {
    /// Create a new Steps
    #[must_use]
    pub const fn new() -> Self {
        Self {
            steps:       Vec::new(),
            current:     0,
            orientation: Orientation::Vertical
        }
    }

    /// Add a step
    #[must_use]
    pub fn add_step(mut self, number: usize, title: &str) -> Self {
        self.steps.push(Step {
            number,
            title: title.to_string(),
            subtitle: None,
            state: StepState::Pending
        });
        self
    }

    /// Set current active step
    #[must_use]
    pub const fn current(mut self, current: usize) -> Self {
        self.current = current;
        self
    }

    /// Set orientation
    #[must_use]
    pub const fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set subtitle for last step
    #[must_use]
    pub fn with_subtitle(mut self, subtitle: &str) -> Self {
        if let Some(step) = self.steps.last_mut() {
            step.subtitle = Some(subtitle.to_string());
        }
        self
    }

    /// Render the steps as HTML string
    #[must_use]
    pub fn render(&self) -> String {
        let orient_class = match self.orientation {
            Orientation::Vertical => "steps--vertical",
            Orientation::Horizontal => "steps--horizontal"
        };

        let mut html = format!(r#"<div class="telegram-ui-steps {orient_class}">"#);

        for (i, step) in self.steps.iter().enumerate() {
            let state_class = if i < self.current {
                "step--completed"
            } else if i == self.current {
                "step--current"
            } else {
                "step--pending"
            };

            let icon = if i < self.current {
                "✓"
            } else {
                &step.number.to_string()
            };

            html.push_str(&format!(
                r#"<div class="step {state_class}">
                    <div class="step-icon">{icon}</div>
                    <div class="step-content">
                        <div class="step-title">{title}</div>"#,
                state_class = state_class,
                icon = icon,
                title = &step.title
            ));

            if let Some(ref subtitle) = step.subtitle {
                html.push_str(&format!(r#"<div class="step-subtitle">{subtitle}</div>"#));
            }

            html.push_str("</div></div>");
        }

        html.push_str("</div>");
        html
    }
}

impl Default for Steps {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Steps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_steps_default() {
        let steps = Steps::new();
        assert!(steps.steps.is_empty());
    }

    #[test]
    fn test_steps_render() {
        let steps = Steps::new()
            .add_step(1, "Step 1")
            .add_step(2, "Step 2")
            .current(1);

        let html = steps.render();
        assert!(html.contains("Step 1"));
        assert!(html.contains("Step 2"));
    }
}
