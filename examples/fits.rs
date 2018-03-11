extern crate fits;
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
    let mut fits_file = fits::Fits::open("/home/malik/workspace/lab/aflak/data/test.fits").unwrap();
    let primary_hdu = &mut fits_file[0];
    let data = primary_hdu.read_data();
    let (shape, image) = match data {
        &fits::FitsData::FloatingPoint32(ref image) => ((image.shape[0], image.shape[1]), {
            let len = image.shape[0] * image.shape[1];
            let nth_frame = 100;
            let start = len * nth_frame;
            let end = len * (nth_frame + 1);
            let mut vec = Vec::new();
            for i in start..end {
                vec.push(image.data[i])
            }
            vec
        }),
        _ => panic!("Expected FloatingPoint32"),
    };
    let image = ImageViewBuilder::new_from_linear_data(image, shape)
        .unwrap()
        .build()
        .unwrap();
    axes.add_child(image);
    fig.add_child(axes);
    app.add_figure(fig);
    app.render().unwrap();
}
