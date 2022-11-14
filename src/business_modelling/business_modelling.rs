use std::{error::Error, fs, ops::Neg};

use crate::business_modelling::linreg;
use chrono::Utc;
use plotters::{
    backend::RGBPixel,
    prelude::{BitMapBackend, ChartBuilder, Circle, IntoDrawingArea, PathElement},
    series::LineSeries,
    style::{full_palette::PURPLE, Color, IntoFont, BLACK, BLUE, GREEN, RED, WHITE},
};
use rust_decimal::prelude::ToPrimitive;

#[derive(Debug)]
pub struct BusinessModelling {}

// Least Squares Method
impl BusinessModelling {
    pub fn scatterplot(
        domain: &Vec<f32>,
        range: &Vec<f32>,
        title: String,
        domain_label: String,
        range_label: String,
        directory: Option<String>,
        height: Option<u32>,
        width: Option<u32>,
    ) -> Result<bool, Box<dyn Error>> {
        let domain_len = domain.len();
        let range_len = range.len();

        if range_len == 0 || domain_len == 0 {
            // Check if domain and range is not empty
            Err("Insufficient series lengths")?;
        } else if range_len != domain_len {
            // Check if domain and range has the same lengths
            Err("Range length is not equal to domain length or vice versa")?;
        }

        // Partner domain points with corresponding range points using zipping of iters
        // points = Using data from above, map each (x, y) tuple into a Green Circle object
        let points = domain.iter().zip(range.iter());

        let circles = points
            .into_iter()
            .map(|(domain, range)| Circle::new((*domain, *range), 10, GREEN.filled()));

        // Get max domain to determine end x of plot
        let max_domain = domain.iter().cloned().fold(-1. / 0. /* -inf */, f32::max) + 1.0;

        // Get max range to determine end y of plot
        let max_range = range.iter().cloned().fold(-1. / 0. /* -inf */, f32::max) + 1.0;

        // Get linear regression or line of best fit (mx, b)
        // regression_points = Calculate regression points using each x or domain as input, output needs to be tuple (domain, regression_point)
        let (m, b) = linreg::Linreg::linear_regress(&domain, &range)?;

        // Setup filepath / directory on which folder to save it
        let dt = Utc::now();
        let timestamp: i64 = dt.timestamp();

        let dir = directory.unwrap_or("chart_outputs".to_string());

        fs::create_dir_all(&dir)?;

        let filepath = format!("{}/{}_scatterplot.png", &dir, timestamp);

        // Build drawing area
        let drawing_area =
            BitMapBackend::new(&filepath, (height.unwrap_or(1024), width.unwrap_or(768)))
                .into_drawing_area();

        drawing_area.fill(&WHITE)?;

        // Set domain spec (Minimum value to Maximum value)
        let x_spec = 0.0..max_domain;
        let y_spec = 0.0..max_range;

        // Set title at top of the graph
        let caption = format!("{} Scatterplot", title);
        let font_style = ("sans-serif", 25.0).into_font();

        // Set x and y labels
        let mut chart_builder = ChartBuilder::on(&drawing_area);
        let mut scatterplot = chart_builder
            .x_label_area_size(40)
            .y_label_area_size(40)
            .caption(caption, font_style)
            .build_cartesian_2d(x_spec, y_spec)?;

        scatterplot
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .x_desc(domain_label)
            .y_desc(range_label)
            .draw()?;

        scatterplot.draw_series(circles)?;

        // Draw line series using regression_points
        let regression_line_data = (0..max_domain.to_i32().unwrap())
            .collect::<Vec<i32>>()
            .iter()
            .map(|x| (x.to_f32().unwrap(), ((m * x.to_f32().unwrap()) + b)))
            .collect::<Vec<(f32, f32)>>();

        let regression_line = LineSeries::new(regression_line_data, PURPLE.stroke_width(2));

        // Fill in moving averages line data series
        scatterplot.draw_series(regression_line)?;

        drawing_area.present().expect(&format!(
            "Cannot write into {:?}. Directory does not exists.",
            &dir
        ));

        println!("Scatterplot has been saved to {}", filepath);

        return Ok(true);
    }

    pub fn demand_supply_scatterplot(
        prices: &Vec<f32>,
        quantity_purchase: &Vec<f32>,
        quantity_produce: &Vec<f32>,
        price: Option<f32>,
        title: String,
        directory: Option<String>,
        height: Option<u32>,
        width: Option<u32>,
    ) -> Result<bool, Box<dyn Error>> {
        let prices_len = prices.len();
        let quantity_purchase_len = quantity_purchase.len();
        let quantity_produce_len = quantity_produce.len();

        if prices_len == 0 || quantity_purchase_len == 0 || quantity_produce_len == 0 {
            // Check if domain and range is not empty
            Err("Insufficient lengths")?;
        } else if (prices_len != quantity_purchase_len) || (prices_len != quantity_produce_len) {
            // Check if domain and range has the same lengths
            Err("Range length is not equal to domain length or vice versa")?;
        }

        // Determine if max demand quantity is > max produce quantity

        let max_demand_qty = &quantity_purchase[0];
        let max_supply_qty = &quantity_produce[prices_len - 1];
        let max_range = if max_demand_qty > max_supply_qty {
            max_demand_qty
        } else {
            max_supply_qty
        };

        // Get demand function
        let (demand_m, demand_b) = linreg::Linreg::linear_regress(&prices, &quantity_purchase)?;

        // Get supply function
        let (supply_m, supply_b) = linreg::Linreg::linear_regress(&prices, &quantity_produce)?;

        // Setup filepath / directory on which folder to save it
        let dt = Utc::now();
        let timestamp: i64 = dt.timestamp();

        let dir = directory.unwrap_or("chart_outputs".to_string());

        fs::create_dir_all(&dir)?;

        let filepath = format!("{}/{}_demand_supply.png", &dir, timestamp);

        // Build drawing area
        let drawing_area =
            BitMapBackend::new(&filepath, (height.unwrap_or(1024), width.unwrap_or(768)))
                .into_drawing_area();

        drawing_area.fill(&WHITE)?;

        let start_price = prices[0];
        let end_price = prices[prices_len - 1];

        let x_spec = start_price..end_price;
        let y_spec = 0.0..max_range.clone();

        // Set title at top of the graph
        let caption = format!("{} Demand and Supply", title);
        let font_style = ("sans-serif", 25.0).into_font();

        // Set x and y labels
        let mut chart_builder = ChartBuilder::on(&drawing_area);
        let mut scatterplot = chart_builder
            .margin(15)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .caption(caption, font_style)
            .build_cartesian_2d(x_spec, y_spec)?;

        scatterplot
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .x_desc("Price")
            .y_desc("Quantity")
            .draw()?;

        let mut demand_regression_line_data = (start_price.to_i32().unwrap()
            ..=end_price.to_i32().unwrap())
            .collect::<Vec<i32>>()
            .iter()
            .map(|x| {
                (
                    x.to_f32().unwrap(),
                    ((demand_m * x.to_f32().unwrap()) + demand_b),
                )
            })
            .collect::<Vec<(f32, f32)>>();

        // Recompute first entry because range doesnt provide floating points
        demand_regression_line_data[0] = (start_price, (demand_m * start_price) + demand_b);

        let demand_regression_line =
            LineSeries::new(demand_regression_line_data, BLUE.stroke_width(2));

        // Draw line series for supply
        let mut supply_regression_line_data = (start_price.to_i32().unwrap()
            ..=end_price.to_i32().unwrap())
            .collect::<Vec<i32>>()
            .iter()
            .map(|x| {
                (
                    x.to_f32().unwrap(),
                    ((supply_m * x.to_f32().unwrap()) + supply_b),
                )
            })
            .collect::<Vec<(f32, f32)>>();

        // Recompute first entry because range doesnt provide floating points
        supply_regression_line_data[0] = (start_price, (supply_m * start_price) + supply_b);

        let supply_regression_line =
            LineSeries::new(supply_regression_line_data, GREEN.stroke_width(2));

        scatterplot
            .draw_series(demand_regression_line)?
            .label("Demand")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

        scatterplot
            .draw_series(supply_regression_line)?
            .label("Supply")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

        // Get demand supply relationship at certain price point
        match price {
            Some(price) => {
                let demand_point = (demand_m * price) + demand_b;
                let supply_point = (supply_m * price) + supply_b;

                let dc = Circle::new((price, demand_point), 5, RED.filled());
                let sc = Circle::new((price, supply_point), 5, RED.filled());

                let circles = [dc, sc];
                let relationship_label = if demand_point > supply_point {
                    "Surplus"
                } else if demand_point == supply_point {
                    "Equilibrium"
                } else {
                    "Shortage"
                };

                scatterplot
                    .draw_series(circles)?
                    .label(relationship_label)
                    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));
            }
            None => {
                println!("No price found for graphing");
            }
        }

        scatterplot
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;

        drawing_area.present().expect(&format!(
            "Cannot write into {:?}. Directory does not exists.",
            &dir
        ));

        println!("Demand and Supply Graph has been saved to {}", filepath);

        return Ok(true);
    }

    pub fn expense_revenue_graph(
        prices: &Vec<f32>,
        quantity_purchase: &Vec<f32>,
        fixed_cost: &f32,
        manufacturing_cost: &f32,
        title: String,
        directory: Option<String>,
        height: Option<u32>,
        width: Option<u32>,
    ) -> Result<bool, Box<dyn Error>> {
        // Get expense function by taking manufacturing cost per product and fixed cost (manufacuring_cost * quantity) + fixed_cost

        // Get demand function constants (demand_m, demand_b)
        let (demand_m, demand_b) = linreg::Linreg::linear_regress(&prices, &quantity_purchase)?;

        // multiply em = manufacturing cost and demand_m, b = multiply demand_b and manufacturing cost
        // em + ec (Expense function)
        let em = manufacturing_cost * demand_m;
        let eb = manufacturing_cost * demand_b;

        // eb = add eb and fixed cost together
        let eb = eb + fixed_cost;

        // Get vertical axis intercept (Set this as end range for y (range))
        // (em * 0) + eb
        let vertical_axis = (em * 0.0) + eb;

        // Get horizontal axis intercept (Set this as end range for x (domain))
        // ec / em
        let horizontal_axis = eb / em.abs();

        println!("HORIZONTAL AXCIS {} {} {}", horizontal_axis, eb, em);

        // Get revenue function by getting the demand function constants and justing making the given domain (price) squared

        // Get max height / max price where revenue will be made. (-b / 2a)
        let axis_of_symmetry_price = demand_b.neg() / (2.0 * demand_m);

        let max_revenue =
            (demand_m * axis_of_symmetry_price.powi(2)) + (demand_b * axis_of_symmetry_price);

        let max_revenue_point = [Circle::new(
            (axis_of_symmetry_price, max_revenue),
            5,
            RED.filled(),
        )];

        let x_spec = 0.0..horizontal_axis;
        let y_spec = 0.0..vertical_axis;

        let expense_line_data = (0..=horizontal_axis.to_i32().unwrap())
            .collect::<Vec<i32>>()
            .iter()
            .map(|x| (x.to_f32().unwrap(), ((em * x.to_f32().unwrap()) + eb)))
            .collect::<Vec<(f32, f32)>>();

        let expense_regression_line = LineSeries::new(expense_line_data, GREEN.stroke_width(2));

        let revenue_line_data = (0..=horizontal_axis.to_i32().unwrap())
            .collect::<Vec<i32>>()
            .iter()
            .map(|x| {
                let x_f32 = x.to_f32().unwrap();

                (
                    x.to_f32().unwrap(),
                    (demand_m * (x_f32 * x_f32)) + (demand_b * x_f32),
                )
            })
            .collect::<Vec<(f32, f32)>>();

        let revenue_regression_line = LineSeries::new(revenue_line_data, PURPLE.stroke_width(2));

        // Setup filepath / directory on which folder to save it
        let dt = Utc::now();
        let timestamp: i64 = dt.timestamp();

        let dir = directory.unwrap_or("chart_outputs".to_string());

        fs::create_dir_all(&dir)?;

        let filepath = format!("{}/{}_expense_revenue.png", &dir, timestamp);

        // Build drawing area
        let drawing_area =
            BitMapBackend::new(&filepath, (height.unwrap_or(1024), width.unwrap_or(768)))
                .into_drawing_area();

        drawing_area.fill(&WHITE)?;

        // Set title at top of the graph
        let caption = format!("{} Expense & Revenue", title);
        let font_style = ("sans-serif", 25.0).into_font();

        // Set x and y labels
        let mut chart_builder = ChartBuilder::on(&drawing_area);
        let mut scatterplot = chart_builder
            .margin(15)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .caption(caption, font_style)
            .build_cartesian_2d(x_spec, y_spec)?;

        scatterplot
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .x_desc("Price")
            .y_desc("Fiat")
            .draw()?;

        scatterplot
            .draw_series(expense_regression_line)?
            .label("Expense")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

        scatterplot
            .draw_series(revenue_regression_line)?
            .label("Revenue")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &PURPLE));

        scatterplot
            .draw_series(max_revenue_point)?
            .label("Max Revenue")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        scatterplot
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;

        drawing_area.present().expect(&format!(
            "Cannot write into {:?}. Directory does not exists.",
            &dir
        ));

        println!("Expense and Revenue Graph has been saved to {}", filepath);

        return Ok(true);
    }
}
