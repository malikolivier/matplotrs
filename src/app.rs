use backend::Backend;
use matplotrs_backend::Backend as BackendTrait;

use figure::Figure;

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

    pub fn render(&mut self) -> Result<i32, <Backend as BackendTrait>::Err> {
        let mut be = Backend::new();
        for fig in self.figs.iter() {
            let title = fig.title().unwrap_or("Figure 1");
            let size = &fig.f.figsize;
            be.new_figure(title, size);
            // TODO axes!
            for artist in fig.children.iter() {
                let path = (**artist).path();
                be.draw_path(&path)?;
            }
        }
        be.show()
    }
}
