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
    let image = ImageViewBuilder::new(vec![vec![0.9; 100]; 100], (100, 100))
        .build()
        .unwrap();
    axes.add_child(image);
    fig.add_child(axes);
    app.add_figure(fig);
    app.render().unwrap();
}
