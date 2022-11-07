use std::error::Error;

use rust_decimal::prelude::ToPrimitive;

#[derive(Debug)]
pub struct Linreg {}

// Least Squares Method
impl Linreg {
    pub fn scatterplot(domain: Vec<f32>, range: Vec<f32>) -> Result<bool, Box<dyn Error>> {
        // Partner domain points with corresponding range points using zipping of iters
        // points = Using data from above, map each (x, y) tuple into a Green Circle object 
        // Get max domain to determine end x of plot

        // Get max range to determine end y of plot

        // Get linear regression or line of best fit (mx, b)
        // regression_points = Calculate regression points using each x or domain as input, output needs to be tuple (domain, regression_point)

        // Build drawing area
        // Set title at top of the graph
        // Set x and y labels

        // chart
        // .configure_mesh()
        // .disable_x_mesh()
        // .disable_y_mesh()
        // .y_desc("Percentage")
        // .x_desc("KAMOTE")
        // .draw()?;

        // Draw line series using regression_points

        // let sma_line = LineSeries::new(ma_line_data, chosen_color.stroke_width(2));

        // // Fill in moving averages line data series
        // chart
        //     .draw_series(sma_line)
        //     .unwrap()
        //     .label(line_series_label)
        //     .legend(legend(chosen_color));


        return Ok(true);
    }
}
