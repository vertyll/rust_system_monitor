use eframe::egui::{Pos2, Response, Sense, Stroke, Ui, Widget, lerp, vec2};

//-----------------------------------------------------------------------------

/// A toggle switch with a sliding knob.
#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub fn toggle(on: &mut bool) -> impl Widget + '_ {
    ToggleSwitchComponent::new(on)
}

//-----------------------------------------------------------------------------

struct ToggleSwitchComponent<'a> {
    on: &'a mut bool,
    width: f32,
    height: f32,
    knob_padding: f32,
}

impl<'a> ToggleSwitchComponent<'a> {
    pub fn new(on: &'a mut bool) -> Self {
        Self {
            on,
            width: 36.0,
            height: 20.0,
            knob_padding: 2.0,
        }
    }
}

impl<'a> Widget for ToggleSwitchComponent<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let (rect, mut response) =
            ui.allocate_exact_size(vec2(self.width, self.height), Sense::click());

        if response.clicked() {
            *self.on = !*self.on;
            response.mark_changed();
        }

        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact_selectable(&response, *self.on);
            let how_on = ui.ctx().animate_bool(response.id, *self.on);

            let rect = rect.expand(visuals.expansion);
            let radius = rect.height() / 2.0;

            ui.painter().rect_filled(rect, radius, visuals.bg_fill);

            let knob_radius = radius - self.knob_padding;
            let center_min = rect.left() + radius;
            let center_max = rect.right() - radius;

            let knob_center = lerp(center_min..=center_max, how_on);

            ui.painter().circle(
                Pos2::new(knob_center, rect.center().y),
                knob_radius,
                visuals.fg_stroke.color,
                Stroke::NONE,
            );
        }

        response
    }
}
