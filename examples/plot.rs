extern crate matplotrs;

use matplotrs::app::App;
use matplotrs::figure::Figure;
use matplotrs::axes::AxesBuilder;
use matplotrs::color::{BLACK};
use matplotrs::line::Line;

fn main() {
    let mut app = App::new();
    let mut fig = Figure::new();
    let mut axes = AxesBuilder::new().with_edgecolor(BLACK).build();
    axes.add_child(Line::new((0.0, 0.0), (0.5, 0.5)));
    fig.add_child(axes);
    app.add_figure(fig);
    app.render().unwrap();
}
