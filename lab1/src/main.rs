use lab1::{binary_search, linear_search};
use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{LineJoin, LineStyle, PointMarker, PointStyle};
use plotlib::view::ContinuousView;
use std::time::Instant;
fn main() {
    let mut c: u64 = 0;
    let mut linear_times_best: Vec<(f64, f64)> = Vec::new();
    let mut binary_times_best: Vec<(f64, f64)> = Vec::new();
    let mut linear_times_worst: Vec<(f64, f64)> = Vec::new();
    let mut binary_times_worst: Vec<(f64, f64)> = Vec::new();
    for n in 0..10 {
        c = c + 10000;
        let vals: Vec<u64> = (0..c).collect();

        let start = Instant::now();
        let index: i32 = linear_search(&vals, 0);
        let duration = start.elapsed().as_secs_f64();
        linear_times_best.push((c as f64, duration));

        let start = Instant::now();
        let index: i32 = linear_search(&vals, c - 1);
        let duration = start.elapsed().as_micros() as f64;
        linear_times_worst.push((c as f64, duration));

        let start = Instant::now();
        let index: i32 = binary_search(&vals, c - 1);
        let duration = start.elapsed().as_secs_f64();
        binary_times_worst.push((c as f64, duration));

        let start = Instant::now();
        let index: i32 = binary_search(&vals, c / 2 - 1);
        let duration = start.elapsed().as_micros() as f64;
        binary_times_best.push((c as f64, duration));
    }
    let linear_plot_worst: Plot = Plot::new(linear_times_worst)
        .legend(String::from("Worst case"))
        .point_style(
            PointStyle::new()
                .colour("black")
                .marker(PointMarker::Circle),
        )
        .line_style(LineStyle::new().colour("red"));

    let linear_plot_best: Plot = Plot::new(linear_times_best)
        .legend(String::from("Best case"))
        .point_style(
            PointStyle::new()
                .colour("black")
                .marker(PointMarker::Circle),
        )
        .line_style(LineStyle::new().linejoin(LineJoin::Round).colour("green"));
    let binary_plot_worst: Plot = Plot::new(binary_times_worst)
        .legend(String::from("Worst case"))
        .point_style(
            PointStyle::new()
                .colour("black")
                .marker(PointMarker::Circle),
        )
        .line_style(LineStyle::new().colour("red").linejoin(LineJoin::Round));
    let binary_plot_best: Plot = Plot::new(binary_times_best)
        .legend(String::from("Best case"))
        .point_style(
            PointStyle::new()
                .colour("black")
                .marker(PointMarker::Circle),
        )
        .line_style(LineStyle::new().colour("green").linejoin(LineJoin::Round));
    let linear_graph_best = ContinuousView::new()
        .add(linear_plot_best)
        .x_range(0., 100000.)
        .y_range(0., 0.0005)
        .x_label("Input-Size")
        .y_label("Execution-Time (second)");

    let linear_graph_worst = ContinuousView::new()
        .add(linear_plot_worst)
        .x_range(0., 100000.)
        .y_range(0., 10000.)
        .x_label("Input-Size")
        .y_label("Execution-Time (microsecond)");

    let binary_graph_worst = ContinuousView::new()
        .add(binary_plot_worst)
        .x_range(0., 100000.)
        .y_range(0., 0.00001)
        .x_label("Input-Size")
        .y_label("Execution-Time (second)");
    let binary_graph_best = ContinuousView::new()
        .add(binary_plot_best)
        .x_range(0., 100000.)
        .y_range(0., 10000.)
        .x_label("Input-Size")
        .y_label("Execution-Time (microsecond)");
    Page::single(&linear_graph_best)
        .save("plottingLinear_best.svg")
        .unwrap();
    Page::single(&linear_graph_worst)
        .save("plottingLinear_worst.svg")
        .unwrap();
    Page::single(&binary_graph_best)
        .save("plottingBinary_best.svg")
        .unwrap();
    Page::single(&binary_graph_worst)
        .save("plottingBinary_worst.svg")
        .unwrap();
}
