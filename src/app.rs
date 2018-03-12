use backend::Backend;
use matplotrs_backend::Backend as BackendTrait;
use matplotrs_backend::{EventKind, FigureId};

use figure::Figure;

pub struct App {
    figs: Vec<FigureContainer>,
}

struct FigureContainer {
    fig: Figure,
    id: Option<FigureId>,
}

impl App {
    pub fn new() -> App {
        App { figs: Vec::new() }
    }

    pub fn add_figure(&mut self, fig: Figure) {
        self.figs.push(FigureContainer { fig, id: None });
    }

    pub fn start(&mut self) -> Result<i32, <Backend as BackendTrait>::Err> {
        let mut be = Backend::new();
        // Init figures
        for fig_container in self.figs.iter_mut() {
            let fig = &fig_container.fig;
            if fig_container.id.is_none() {
                fig_container.id = Some(fig.create(&mut be)?);
            }
        }
        // Event loop
        while let Some(event) = be.next_event() {
            match event.e {
                EventKind::Render => {
                    let maybe_fig = self.figure_by_id(event.fig_id);
                    match maybe_fig {
                        Some(ref fig) => fig.render(&mut be, event.fig_id)?,
                        None => eprintln!("[WARN] Could not find figure with given ID..."),
                    }
                }
                EventKind::Update(_dt) => (), /* NOOP for the time being */
                EventKind::SaveToFile => be.save_to_file()?,
            };
        }
        Ok(0)
    }

    fn figure_by_id(&self, id: FigureId) -> Option<&Figure> {
        for fig_container in self.figs.iter() {
            if fig_container.id.is_some() && fig_container.id.unwrap() == id {
                return Some(&fig_container.fig);
            }
        }
        None
    }
}
