use chrono::prelude::*;
use log::info;
use std::f32::consts::PI;
use std::time::Duration;

use kas::TkAction;
use kas::draw::{Colour, DrawRounded, DrawText};
use kas::geom::{Quad, Vec2};
use kas::text::util::set_text_and_prepare;
use kas::widget::Window;
use kas::{event, prelude::*};
use kas_wgpu::draw::DrawWindow;

#[derive(Clone, Debug, kas :: macros :: Widget)]
#[handler(handle=noauto)]
#[widget(config = noauto)]
struct Clock {
    #[widget_core]
    core: kas::CoreData,
    date_pos: Coord,
    time_pos: Coord,
    now: DateTime<Local>,
    date: Text<String>,
    time: Text<String>,
}

impl Layout for Clock {
    fn size_rules(&mut self, _: &mut dyn SizeHandle, _: AxisInfo) -> SizeRules {
        SizeRules::new(100, 200, (0, 0), StretchPolicy::HighUtility)
    }

    #[inline]
    fn set_rect(&mut self, rect: Rect, _align: AlignHints) {
        // Force to square
        let size = rect.size.0.min(rect.size.1);
        let size = Size(size, size);
        let excess = rect.size - size;
        let pos = rect.pos + (excess * 0.5);
        self.core.rect = Rect { pos, size };

        // Note: font size is calculated as dpem = dpp * pt_size. Instead of
        // the usual semantics we set dpp=1 (in constructor) and pt_size=dpem.
        let dpem = (size.1 as f32 * 0.12).into();
        let half_size = Size(size.0, size.1 / 2);
        self.date.update_env(|env| {
            env.set_pt_size(dpem);
            env.set_bounds(half_size.into());
        });
        self.time.update_env(|env| {
            env.set_pt_size(dpem);
            env.set_bounds(half_size.into());
        });
        self.date_pos = pos + Size(0, size.1 - half_size.1);
        self.time_pos = pos;
    }

    fn draw(&self, draw_handle: &mut dyn DrawHandle, _: &ManagerState, _: bool) {
        let col_face = Colour::grey(0.4);
        let col_hands = Colour::new(0.2, 0.2, 0.4);
        let col_secs = Colour::new(0.6, 0.2, 0.2);
        let col_text = Colour::grey(0.0);

        // We use the low-level draw device to draw our clock. This means it is
        // not themeable, but gives us much more flexible draw routines.
        //
        // Note: offset is used for scroll-regions, and should be zero here;
        // we add it anyway as is recommended.
        let (pass, offset, draw) = draw_handle.draw_device();
        let draw = draw.as_any_mut().downcast_mut::<DrawWindow<()>>().unwrap();

        let rect = Quad::from(self.core.rect + offset);
        draw.circle(pass, rect, 0.95, col_face);

        let half = (rect.b.1 - rect.a.1) / 2.0;
        let centre = rect.a + half;

        let mut line_seg = |t: f32, r1: f32, r2: f32, w, col| {
            let v = Vec2(t.sin(), -t.cos());
            draw.rounded_line(pass, centre + v * r1, centre + v * r2, w, col);
        };

        let w = half * 0.015625;
        let l = w * 5.0;
        let r = half - w;
        for d in 0..12 {
            let l = if d % 3 == 0 { 2.0 * l } else { l };
            line_seg(d as f32 * (PI / 6.0), r - l, r, w, col_face);
        }

        let secs = self.now.time().num_seconds_from_midnight();
        let a_sec = (secs % 60) as f32 * (PI / 30.0);
        let a_min = (secs % 3600) as f32 * (PI / 1800.0);
        let a_hour = (secs % 43200) as f32 * (PI / (21600.0));

        line_seg(a_hour, 0.0, half * 0.55, half * 0.03, col_hands);
        line_seg(a_min, 0.0, half * 0.8, half * 0.015, col_hands);
        line_seg(a_sec, 0.0, half * 0.9, half * 0.005, col_secs);

        let pos = (self.date_pos + offset).into();
        let bounds = self.date.env().bounds.into();
        draw.text(pass, pos, bounds, Vec2::ZERO, self.date.as_ref(), col_text);
        let pos = (self.time_pos + offset).into();
        let bounds = self.time.env().bounds.into();
        draw.text(pass, pos, bounds, Vec2::ZERO, self.time.as_ref(), col_text);
    }
}

impl WidgetConfig for Clock {
    fn configure(&mut self, mgr: &mut Manager) {
        mgr.update_on_timer(Duration::new(0, 0), self.id());
    }
}

impl Handler for Clock {
    type Msg = event::VoidMsg;

    #[inline]
    fn handle(&mut self, mgr: &mut Manager, event: Event) -> Response<Self::Msg> {
        match event {
            Event::TimerUpdate => {
                self.now = Local::now();
                let date = self.now.format("%Y-%m-%d").to_string();
                let time = self.now.format("%H:%M:%S").to_string();
                // let avail = Size(self.core.rect.size.0, self.core.rect.size.1 / 2);
                *mgr += TkAction::Redraw
                    + set_text_and_prepare(&mut self.date, date)
                    + set_text_and_prepare(&mut self.time, time);
                let ns = 1_000_000_000 - (self.now.time().nanosecond() % 1_000_000_000);
                info!("Requesting update in {}ns", ns);
                mgr.update_on_timer(Duration::new(0, ns), self.id());
                Response::None
            }
            event => Response::Unhandled(event),
        }
    }
}

impl Clock {
    fn new() -> Self {
        let env = kas::text::Environment {
            align: (Align::Centre, Align::Centre),
            dpp: 1.0,
            ..Default::default()
        };
        let date = Text::new(env.clone(), "0000-00-00".into());
        let time = Text::new(env, "00:00:00".into());
        Clock {
            core: Default::default(),
            date_pos: Coord::ZERO,
            time_pos: Coord::ZERO,
            now: Local::now(),
            date,
            time,
        }
    }
}

fn main() -> Result<(), kas_wgpu::Error> {
    env_logger::init();

    let window = Window::new("Clock", Clock::new());

    let theme = kas_theme::FlatTheme::new();
    kas_wgpu::Toolkit::new(theme)?.with(window)?.run()
}