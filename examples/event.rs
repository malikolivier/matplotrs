extern crate matplotrs;

use matplotrs::app::App;
use matplotrs::figure::FigureBuilder;
use matplotrs::axes::AxesBuilder;
use matplotrs::color::{BLACK, GREEN, RED};

fn main() {
    let mut app = App::new();
    let mut fig = FigureBuilder::new()
        .with_figsize(200, 200)
        .with_title("Empty!")
        .build();
    fig.onclick(|e| {
        println!("{:?}", e);
        fig.set_facecolor(RED);
    });
    app.add_figure(fig);
    app.start().unwrap();
}
