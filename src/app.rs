use figure::Figure;
use render::RenderError;

pub struct App {
    pub figs: Vec<Figure>
}

impl App {
    pub fn new() -> App {
        App { figs: Vec::new() }
    }

    pub fn add_figure(&mut self, fig: Figure) {
        self.figs.push(fig);
    }

    pub fn render(&mut self) -> Result<(), RenderError> {

        for fig in self.figs.iter() {

        }
        Ok(())
    }
}
