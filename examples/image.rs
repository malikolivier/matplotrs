extern crate matplotrs;

use matplotrs::app::App;
use matplotrs::figure::Figure;
use matplotrs::axes::AxesBuilder;
use matplotrs::color::BROWN;
use matplotrs::image_view::ImageViewBuilder;

fn main() {
    let mut app = App::new();
    let mut fig = Figure::new();
    let mut axes = AxesBuilder::new().with_edgecolor(BROWN).build();
    let image = ImageViewBuilder::new(vec![vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9]; 100])
        .with_ylims((-10.0, 140.0))
        .with_xlims((-6.0, 14.0))
        .build()
        .unwrap();
    axes.add_child(image);
    fig.add_child(axes);
    app.add_figure(fig);
    app.render().unwrap();
}
