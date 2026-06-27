use plotters::prelude::*;
use statrs::distribution::{Normal, Continuous};

const DICE_1_MAX: usize = 20;
const DICE_2_MAX: usize = 20;
const DICE_1_MIN: usize = 1;
const DICE_2_MIN: usize = 1;

const ROUNDS: usize = 1000;

const EPOCH: usize = 1000;

fn main(){

    let mut sample_means: Vec<f64> = Vec::new();
    let mut all_samples: Vec<f64> = Vec::new();

    for _ in 0..EPOCH{

        let mut histogram_vector: Vec<f64> = Vec::new();
    
        for _ in 0..ROUNDS{
            let dice_1_random_number = rand::random_range(DICE_1_MIN..DICE_1_MAX+1) as f64;
            let dice_2_random_number = rand::random_range(DICE_2_MIN..DICE_2_MAX+1) as f64;
            histogram_vector.push(dice_1_random_number + dice_2_random_number);
        }

        let (sample_mu, _sample_stdev) = generate_mean_and_stdev(&histogram_vector);

        sample_means.push(sample_mu);
        all_samples.append(&mut histogram_vector);

    }

    _ = draw_histogram(&all_samples, "dice_histogram.png".to_string());
    _ = draw_normal(&all_samples, "dice_nomrmal_distrobution.png".to_string());
    _ = draw_normal(&sample_means, "clt_example.png".to_string());

}

fn generate_mean_and_stdev(data:&[f64]) -> (f64, f64){
    let mean = data.iter().sum::<f64>()/ data.len() as f64;
    let stdev = (data.iter().map(|x| (x-mean).powi(2)).sum::<f64>()).sqrt();
    return (mean, stdev);
}

fn draw_histogram(data:&[f64],file_output:String)-> Result<(), Box<dyn std::error::Error>>{


    let data_u32: Vec<u16> = data.iter().map(|&x| x as u16).collect();


    let distribution_maximum: usize = DICE_1_MAX + DICE_2_MAX;
    let distribution_minimum: usize = DICE_1_MIN + DICE_2_MIN;
    
    let root = BitMapBackend::new(&file_output, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .margin(5)
        // .caption("Histogram Test", ("sans-serif", 50.0))
        .build_cartesian_2d(distribution_minimum ..distribution_maximum , 0f64..200000 as f64)?;

    chart
        .configure_mesh()
        .draw()?;

    chart
        .draw_series(Histogram::vertical(&chart)
        .style(RED.mix(0.5).filled())
        .data(data_u32.iter().map(|x: &u16| (*x, 1.0))),
    )?;

    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");

    Ok(())

}

fn draw_normal(data:&[f64], file_output: String) -> Result<(), Box<dyn std::error::Error>> {

    let (sample_means_mu, sample_means_stdev) = generate_mean_and_stdev(&data);

    println!("mu = {}, stdev = {}", sample_means_mu, sample_means_stdev);

    let normal_distribution = Normal::new(sample_means_mu, sample_means_stdev).unwrap();

    // Set the bounds of the graph to be 4 stdevs above and below the mean
    let x_min = sample_means_mu - 4.0 * sample_means_stdev;
    let x_max = sample_means_mu + 4.0 * sample_means_stdev;

    let max_pdf = normal_distribution.pdf(sample_means_mu);

    let root = BitMapBackend::new(&file_output, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .build_cartesian_2d(x_min..x_max, 0.0..(1.0 / (sample_means_stdev * (2.0 * std::f64::consts::PI).sqrt())))?;

    chart
        .configure_mesh()
        .draw()?;

    chart
        .draw_series(LineSeries::new(
            (0..1000)
                .map(|i| {
                    let x = x_min + (x_max - x_min) * (i as f64 / 1000.0);
                    (x, normal_distribution.pdf(x))
                }),
            &RED,
        ))?
        .label(format!("μ={:.2}, σ={:.2}", sample_means_mu, sample_means_stdev))
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart.draw_series(std::iter::once(PathElement::new(
        vec![(sample_means_mu, 0.0), (sample_means_mu, max_pdf * 1.2)],
        &BLACK,
    )))?;

    chart.configure_series_labels().border_style(&BLACK).draw()?;

    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");

    Ok(())

}