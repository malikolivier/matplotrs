extern crate matplotrs;

use matplotrs::app::App;
use matplotrs::figure::FigureBuilder;
use matplotrs::color::BLACK;

fn main() {
    let mut app = App::new();
    run(&mut app)
}

fn run(app: &mut App) {
    let mut fig = FigureBuilder::new().with_figsize(200, 300).with_title("Empty!").build();
    fig.add_axes().with_rect(&[0.1, 0.1, 0.8, 0.8]).with_facecolor(BLACK).build();
    app.add_figure(fig);
    // Render!
    app.render().unwrap();
}
