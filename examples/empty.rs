extern crate matplotrs;
//extern crate matplotrs_backend;

use matplotrs::app::App;
//use matplotrs::figure::FigureBuilder;
use matplotrs::color::BLACK;

// use matplotrs::dummy_backend::DummyBackend;
// use matplotrs_backend::Backend;

fn main() {
    let mut app = App::new();
    run(&mut app)
}

fn run(app: &mut App) {
    let fig = app.add_figure().with_figsize(200, 300).with_title("Empty!").build();
    fig.add_axes().with_rect(&[0.1, 0.1, 0.8, 0.8]).with_facecolor(BLACK).build();

    // Render!
    app.render().unwrap();
}
