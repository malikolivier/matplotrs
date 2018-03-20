extern crate matplotrs;

use matplotrs::app::App;
use matplotrs::figure::FigureBuilder;
use matplotrs::color::{RED, WHITE};

fn main() {
    let mut app = App::new();
    let mut fig = FigureBuilder::new()
        .with_figsize(200, 200)
        .with_title("Empty!")
        .build();
    let mut test = false;
    fig.onclick(move |e, figs| {
        println!("{:?}", e);
        if test {
            figs[0].facecolor = WHITE;
        } else {
            figs[0].facecolor = RED;
        }
        test = !test;
    });
    app.add_figure(fig);
    app.start().unwrap();
}
