extern crate matplotrs;

use matplotrs::app::App;
use matplotrs::figure::Figure;
use matplotrs::axes::AxesBuilder;
use matplotrs::color::RED;
use matplotrs::line::Line;
use matplotrs::line_collection::LineCollectionBuilder;
use matplotrs::plot::PlotBuilder;


fn main() {
    let mut app = App::new();
    let mut fig = Figure::new();
    let mut axes = AxesBuilder::new().with_edgecolor(RED).build();
    axes.add_child(Line::new((0.0, 0.0), (0.5, 0.5)));
    let linecol = LineCollectionBuilder::new().with_vertices(
        vec![(-1.0, -0.7), (-0.7, -0.6), (-0.4, -0.75), (-0.1, -0.8)]
    ).build();
    axes.add_child(linecol);
    let plot = PlotBuilder::new(vec![(0.0, 1.0), (1.0, 1.5), (2.0, 1.7)]).build();
    axes.add_child(plot);
    fig.add_child(axes);
    app.add_figure(fig);
    app.render().unwrap();
}
