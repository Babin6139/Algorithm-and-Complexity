use lab2::{insertion_sort, merge_sort};
use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{LineJoin, LineStyle, PointMarker, PointStyle};
use plotlib::view::ContinuousView;
use std::time::Instant;

fn main() {
    let mut c: u64 = 0;
    let mut insertionsort_times_best: Vec<(f64, f64)> = Vec::new();
    let mut insertionsort_times_worst: Vec<(f64, f64)> = Vec::new();
    let mut merge_sort_time: Vec<(f64, f64)> = Vec::new();
    for _n in 0..20 {
        c = c + 5000;
        let mut vals: Vec<u64> = (0..c).collect();

        let start = Instant::now();
        insertion_sort(&mut vals);
        let duration = start.elapsed().as_micros() as f64;
        insertionsort_times_best.push((c as f64, duration));
        let length = vals.len() - 1;
        let _ = &mut vals.reverse();
        let start = Instant::now();
        insertion_sort(&mut vals);
        let duration = start.elapsed().as_millis() as f64;
        insertionsort_times_worst.push((c as f64, duration));
        let start = Instant::now();
        merge_sort(&mut vals, 0, length);
        let duration = start.elapsed().as_millis() as f64;
        merge_sort_time.push((c as f64, duration));
    }
    let insertion_plot_worst: Plot = Plot::new(insertionsort_times_worst)
        .legend(String::from("Worst case"))
        .point_style(
            PointStyle::new()
                .colour("black")
                .marker(PointMarker::Circle),
        )
        .line_style(LineStyle::new().colour("red"));

    let insertion_plot_best: Plot = Plot::new(insertionsort_times_best)
        .legend(String::from("Best case"))
        .point_style(
            PointStyle::new()
                .colour("black")
                .marker(PointMarker::Circle),
        )
        .line_style(LineStyle::new().linejoin(LineJoin::Round).colour("green"));
    let merge_plot: Plot = Plot::new(merge_sort_time)
        .legend(String::from("Best case"))
        .point_style(
            PointStyle::new()
                .colour("black")
                .marker(PointMarker::Circle),
        )
        .line_style(LineStyle::new().colour("green").linejoin(LineJoin::Round));
    let insertion_graph_best = ContinuousView::new()
        .add(insertion_plot_best)
        .x_range(0., 100000.)
        .y_range(0., 10000.)
        .x_label("Input-Size")
        .y_label("Execution-Time (microsecond)");

    let insertion_graph_worst = ContinuousView::new()
        .add(insertion_plot_worst)
        .x_range(0., 100000.)
        .y_range(0., 60000.)
        .x_label("Input-Size")
        .y_label("Execution-Time (millisecond)");

    let merge_graph = ContinuousView::new()
        .add(merge_plot)
        .x_range(0., 100000.)
        .y_range(0., 300.)
        .x_label("Input-Size")
        .y_label("Execution-Time (millisecond)");

    Page::single(&insertion_graph_best)
        .save("insertion_best.svg")
        .unwrap();
    Page::single(&insertion_graph_worst)
        .save("insertion_worst.svg")
        .unwrap();
    Page::single(&merge_graph).save("mergeSort.svg").unwrap();
}
