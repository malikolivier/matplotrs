extern crate matplotrs;

use matplotrs::app::App;
use matplotrs::figure::FigureBuilder;
use matplotrs::axes::AxesBuilder;
use matplotrs::color::BLACK;

fn main() {
    let mut app = App::new();
    run(&mut app)
}

fn run(app: &mut App) {
    let mut fig = FigureBuilder::new().with_figsize(200, 200).with_title("Empty!").build();
    let axes = AxesBuilder::new().with_rect(&[0.1, 0.1, 0.8, 0.8]).with_facecolor(BLACK).build();
    fig.add_child(axes);
    app.add_figure(fig);
    let fig2 = FigureBuilder::new().with_figsize(100, 120).with_title("Empty2!").build();
    app.add_figure(fig2);
    // Render!
    app.render().unwrap();
}
