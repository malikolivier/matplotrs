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

type BackEndError = <Backend as BackendTrait>::Err;

impl App {
    pub fn new() -> App {
        App { figs: Vec::new() }
    }

    pub fn add_figure(&mut self, fig: Figure) {
        self.figs.push(FigureContainer { fig, id: None });
    }

    pub fn start(&mut self) -> Result<i32, BackEndError> {
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
                EventKind::Render => self.map_on_figure_by_id_safe(event.fig_id, |fig| {
                    fig.render(&mut be, event.fig_id)
                })?,
                EventKind::Update(_dt) => (), /* NOOP for the time being */
                EventKind::SaveToFile => be.save_to_file()?,
                EventKind::Resize(w, h) => self.map_on_figure_by_id(event.fig_id, |fig| {
                    fig.set_figsize(w, h);
                })?,
                EventKind::Close => (), /* NOOP for the time being */
                EventKind::Click(e) => {
                    println!("{:?}", e);
                    self.map_on_figure_by_id(event.fig_id, |fig| {
                        for handler in fig.f.click_event_handlers.iter_mut() {
                            handler(&e);
                        }
                    })?;
                }
            };
        }
        Ok(0)
    }

    fn figure_by_id(&mut self, id: FigureId) -> Option<&mut Figure> {
        for fig_container in self.figs.iter_mut() {
            if fig_container.id.is_some() && fig_container.id.unwrap() == id {
                return Some(&mut fig_container.fig);
            }
        }
        None
    }

    fn map_on_figure_by_id<F, U>(&mut self, id: FigureId, mut f: F) -> Result<U, String>
    where
        F: FnMut(&mut Figure) -> U,
    {
        let mut maybe_fig = self.figure_by_id(id);
        match maybe_fig {
            Some(ref mut fig) => Ok(f(fig)),
            None => Err(FIGURE_NOT_FOUND_ERR.to_owned()),
        }
    }

    fn map_on_figure_by_id_safe<F, U, E>(&mut self, id: FigureId, mut f: F) -> Result<U, E>
    where
        F: FnMut(&mut Figure) -> Result<U, E>,
        E: From<&'static str>,
    {
        let mut maybe_fig = self.figure_by_id(id);
        match maybe_fig {
            Some(ref mut fig) => Ok(f(fig)?),
            None => Err(From::from(FIGURE_NOT_FOUND_ERR)),
        }
    }
}

const FIGURE_NOT_FOUND_ERR: &str = "Could not find figure with given ID...";
