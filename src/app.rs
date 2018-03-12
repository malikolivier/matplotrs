use backend::Backend;
use matplotrs_backend::Backend as BackendTrait;
use matplotrs_backend::Event;

use figure::Figure;

pub struct App {
    figs: Vec<FigureContainer>,
}

struct FigureContainer {
    fig: Figure,
    created: bool,
}

impl App {
    pub fn new() -> App {
        App { figs: Vec::new() }
    }

    pub fn add_figure(&mut self, fig: Figure) {
        self.figs.push(FigureContainer {
            fig,
            created: false,
        });
    }

    pub fn start(&mut self) -> Result<i32, <Backend as BackendTrait>::Err> {
        let mut be = Backend::new();
        while let Some(event) = be.next_event() {
            match event {
                Event::Render => self.render(&mut be)?,
                Event::SaveToFile => be.save_to_file()?,
            };
        }
        Ok(0)
    }

    fn render(&mut self, be: &mut Backend) -> Result<(), <Backend as BackendTrait>::Err> {
        for fig_container in self.figs.iter_mut() {
            let fig = &fig_container.fig;
            if !fig_container.created {
                fig.create(be)?;
                fig_container.created = true;
            }
            fig.render(be)?;
        }
        Ok(())
    }
}
