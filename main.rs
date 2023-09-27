// cargo run

use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{PointMarker, PointStyle};
use plotlib::view::ContinuousView;

fn main() {
    // Scatter plots expect a list of pairs
    let data1 = vec![
        (0.1, 0.1),
        (0.2, 0.2),
        (0.3, 0.3),
        (0.4, 0.4),
        (0.5, 0.5),
        (0.6, 0.6),
        (0.7, 0.7),
        (0.8, 0.7),
        (0.9, 0.8),
        (1.0, 0.9),
        (1.1, 0.9),
        (1.2, 1.0),
        (1.3, 1.0),
        (1.4, 1.1),
        (1.5, 1.1),
        (1.6, 1.1),
        (1.7, 1.1),
        (1.8, 1.1),
        (1.9, 1.1),
        (2.0, 1.0),
        (2.1, 1.0),
        (2.2, 0.9),
        (2.3, 0.9),
        (2.4, 0.8),
        (2.5, 0.7),
        (2.6, 0.6),
        (2.7, 0.5),
        (2.8, 0.4),
        (2.9, 0.3),
        (3.0, 0.2),
    ];
    // We create our scatter plot from the data
    let s1: Plot = Plot::new(data1).point_style(
        PointStyle::new().marker(PointMarker::Square), // setting the marker to be a square .colour("#DD3355")
    ); // and a custom colour // We can plot multiple data sets in the same view
    let data2 = vec![ // True solve:
        (0.1, 0.1),
        (0.2, 0.2),
        (0.3, 0.3),
        (0.4, 0.4),
        (0.5, 0.5),
        (0.6, 0.6),
        (0.7, 0.6),
        (0.8, 0.7),
        (0.9, 0.8),
        (1.0, 0.8),
        (1.1, 0.9),
        (1.2, 0.9),
        (1.3, 1.0),
        (1.4, 1.0),
        (1.5, 1.0),
        (1.6, 1.0),
        (1.7, 1.0),
        (1.8, 1.0),
        (1.9, 0.9),
        (2.0, 0.9),
        (2.1, 0.9),
        (2.2, 0.8),
        (2.3, 0.7),
        (2.4, 0.7),
        (2.5, 0.6),
        (2.6, 0.5),
        (2.7, 0.4),
        (2.8, 0.3),
        (2.9, 0.2),
        (3.0, 0.1),
    ];
    let s2: Plot = Plot::new(data2).point_style(
        PointStyle::new(), // uses the default marker .colour("#35C788")
    ); // and a different colour // The 'view' describes what set of data is drawn
    let v = ContinuousView::new()
        .add(s1)
        .add(s2)
        .x_range(0., 3.5)
        .y_range(0., 1.2)
        .x_label("x")
        .y_label("y(x) = sin(x)");
    // A page with a single view is then saved to an SVG file
    Page::single(&v).save("plot.svg").unwrap();
}
