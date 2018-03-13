extern crate matplotrs;

use matplotrs::app::App;
use matplotrs::figure::FigureBuilder;
use matplotrs::axes::AxesBuilder;
use matplotrs::color::{RED, WHITE, YELLOW};
use matplotrs::line::Line;
use matplotrs::line_collection::LineCollectionBuilder;
use matplotrs::plot::PlotBuilder;

fn main() {
    let mut app = App::new();
    let mut fig = FigureBuilder::new().with_facecolor(YELLOW).build();
    let mut axes = AxesBuilder::new()
        .with_edgecolor(RED)
        .with_facecolor(WHITE)
        .build();
    axes.add_child(Line::new((0.0, 0.0), (0.5, 0.5)));
    let linecol = LineCollectionBuilder::new()
        .with_vertices(vec![
            (-1.0, -0.7),
            (-0.7, -0.6),
            (-0.4, -0.75),
            (-0.1, -0.8),
        ])
        .build();
    axes.add_child(linecol);
    let plot = PlotBuilder::new(vec![(0.0, 1.0), (1.0, 1.5), (2.0, 1.7)]).build();
    axes.add_child(plot);
    fig.add_child(axes);
    let linecol_fig = LineCollectionBuilder::new()
        .with_vertices(vec![
            (-1.0, -0.7),
            (-0.7, -0.6),
            (-0.4, -0.75),
            (-0.1, -0.8),
        ])
        .build();
    fig.add_child(linecol_fig);
    app.add_figure(fig);
    app.start().unwrap();
}
