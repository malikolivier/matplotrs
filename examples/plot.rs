extern crate matplotrs;

use matplotrs::app::App;
use matplotrs::figure::Figure;
use matplotrs::axes::AxesBuilder;
use matplotrs::color::{BLACK};

fn main() {
    let mut app = App::new();
    let mut fig = Figure::new();
    let axes = AxesBuilder::new().with_edgecolor(BLACK).build();
    fig.add_child(axes);
    app.add_figure(fig);
    app.render().unwrap();
}
