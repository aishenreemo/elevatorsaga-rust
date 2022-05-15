use crate::config::Config;
use crate::utils;
use crate::Error;

use sdl2::rect::Rect;
use sdl2::surface::Surface;

use std::time::SystemTime;

pub struct Game {
    pub timestamp: SystemTime,
    pub floors: Vec<Floor>,
    pub elevators: Vec<Elevator>,
    pub layout: Layout,
}

pub struct Floor {
    pub layout: FloorLayout,
}

pub struct Elevator {
    pub position: usize,
    pub capacity: usize,
    pub layout: ElevatorLayout,
}

pub struct Layout {
    pub window: Rect,
    pub top_area: Rect,
    pub main_area: Rect,
    pub log_area: Rect,
}

pub struct FloorLayout {
    pub rect: Rect,
    pub label_rect: Rect,
    pub label: Surface<'static>,
}

pub struct ElevatorLayout {
    pub rect: Rect,
}

impl Game {
    pub fn new(cfg: &Config) -> Result<Self, Error> {
        let layout = Layout::new(cfg);
        Ok(Self {
            timestamp: SystemTime::now(),
            floors: Floor::init(&layout, cfg)?,
            elevators: Elevator::init(&layout, cfg),
            layout,
        })
    }
}

impl Floor {
    fn init(layout: &Layout, cfg: &Config) -> Result<Vec<Self>, Error> {
        let mut out = vec![];

        let mas = (layout.main_area.width(), layout.main_area.height());
        let size = (mas.0 as f32 * 0.75, mas.1 as f32 / cfg.floors_length as f32);
        let label_size = (
            (mas.0 as f32 * 0.04) as u32,
            std::cmp::min(mas.1 / 10, size.1 as u32),
        );

        let x = layout.main_area.x();
        let label_x = x as u32 + mas.0 as u32 / 100;
        for i in 0..cfg.floors_length {
            let y = (cfg.floors_length - (i + 1)) as i32;
            let y = y * size.1 as i32;
            let y = y + layout.main_area.y();

            let label_y = y as u32;

            let rect = Rect::new(x, y, size.0 as u32, size.1 as u32);
            let size = (label_size.0 as u32, size.1 as u32);
            let label_rect_center = utils::find_center_point(size, (label_x, label_y));
            let label_rect_center = (label_rect_center.0 as i32, label_rect_center.1 as i32);
            let label_rect = Rect::from_center(label_rect_center, label_size.0, label_size.1);
            let label = FloorLayout::get_surface(i, cfg)?;

            let layout = FloorLayout::new(rect, label_rect, label);
            let floor = Floor { layout };

            out.push(floor);
        }

        Ok(out)
    }
}

impl Elevator {
    fn new(position: usize, capacity: usize, layout: ElevatorLayout) -> Self {
        Self {
            position,
            capacity,
            layout,
        }
    }

    fn init(layout: &Layout, cfg: &Config) -> Vec<Self> {
        let mut out = vec![];

        let position = 0;
        let capacity = 4;
        let mas = (layout.main_area.width(), layout.main_area.height());

        let height = mas.1 as f32 / cfg.floors_length as f32;
        let elevator_height = std::cmp::min((mas.1 as f32 * 0.1) as u32, height as u32);
        let elevator_width = (mas.0 as f32 * 0.01) as u32 * capacity as u32;
        let elevator_gap = (mas.0 as f32 * 0.03) as u32;

        for i in 0..cfg.elevators_length {
            let y = (cfg.floors_length - position) as f32 * height;
            let y = layout.main_area.y() as u32 + y as u32;
            let y = y - elevator_height as u32;

            let x = layout.main_area.x() as u32 + (mas.0 as f32 * 0.25) as u32;
            let x = x + (elevator_width + elevator_gap) * i as u32;

            let center = utils::find_center_point((elevator_width, elevator_height), (x, y));
            let center = (center.0 as i32, center.1 as i32);

            let rect = Rect::from_center(center, elevator_width, elevator_height);
            let layout = ElevatorLayout { rect };
            let elevator = Elevator::new(position, capacity, layout);

            out.push(elevator);
        }

        out
    }
}

impl Layout {
    fn new(cfg: &Config) -> Self {
        let ws = cfg.window_size;
        let tas = (ws.0, (ws.1 as f32 * 0.9) as u32);
        let mas = ((tas.0 as f32 * 0.9) as u32, (tas.1 as f32 * 0.8) as u32);
        let las = ((mas.0 as f32 * 0.25) as u32, mas.1);

        let ma_center = utils::find_center_point(tas, (0, 0));
        let ma_center = (ma_center.0 as i32, ma_center.1 as i32);
        let window = Rect::new(0, 0, ws.0, ws.1);
        let top_area = Rect::new(0, 0, tas.0, tas.1);
        let main_area = Rect::from_center(ma_center, mas.0, mas.1);
        let log_area = Rect::new(
            main_area.x() + (mas.0 as f32 * 0.75) as i32,
            main_area.y(),
            las.0,
            las.1,
        );

        Self {
            window,
            top_area,
            main_area,
            log_area,
        }
    }
}

impl FloorLayout {
    fn new(rect: Rect, label_rect: Rect, label: Surface<'static>) -> Self {
        Self {
            rect,
            label_rect,
            label,
        }
    }

    fn get_surface(i: usize, cfg: &Config) -> Result<Surface<'static>, Error> {
        Ok(cfg
            .fonts
            .mangonel
            .render(&i.to_string())
            .blended(cfg.palette.blue)?)
    }
}
