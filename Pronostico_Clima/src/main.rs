use reqwest::Error;
use serde::Deserialize;
use chrono::{Local, NaiveDate, NaiveDateTime};
use plotters::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let latitude = -33.215544441886905;
    let longitude = -70.66355361024546;

    // URL de la API de Open-Meteo
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&hourly=temperature_2m&timezone=auto",
        latitude, longitude
    );

    // Realizar solicitud HTTP GET
    let response = reqwest::get(&url).await?;
    if response.status().is_success() {
        let forecast: WeatherResponse = response.json().await?;
        println!("Generando gráfico para las coordenadas ({}, {})...", latitude, longitude);

        // Obtener la fecha actual
        let today = Local::now().date_naive();

        // Filtrar temperaturas de hoy y mañana
        let mut today_temps = Vec::new();
        let mut tomorrow_temps = Vec::new();
        let mut hours_today = Vec::new();
        let mut hours_tomorrow = Vec::new();

        for (time, temp) in forecast.hourly.time.iter().zip(forecast.hourly.temperature_2m.iter()) {
            if let Ok(datetime) = NaiveDateTime::parse_from_str(time, "%Y-%m-%dT%H:%M") {
                if datetime.date() == today {
                    today_temps.push(*temp);
                    hours_today.push(datetime.time().format("%H:%M").to_string());
                } else if datetime.date() == today.succ() {
                    tomorrow_temps.push(*temp);
                    hours_tomorrow.push(datetime.time().format("%H:%M").to_string());
                }
            }
        }

        // Crear el gráfico
        let root = BitMapBackend::new("temperatura.png", (1024, 768)).into_drawing_area();
        root.fill(&WHITE).unwrap();
        let mut chart = ChartBuilder::on(&root)
            .caption("Temperatura: Hoy y Mañana", ("sans-serif", 50).into_font())
            .margin(10)
            .x_label_area_size(50)
            .y_label_area_size(50)
            .build_cartesian_2d(0..24, *today_temps.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(&0.0) as i32
                ..*today_temps.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(&40.0) as i32)
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        chart
            .draw_series(LineSeries::new(
                today_temps.iter().enumerate().map(|(i, &temp)| (i as i32, temp as i32)),
                &RED,
            ))
            .unwrap()
            .label("Hoy")
            .legend(|(x, y)| PathElement::new([(x, y), (x + 20, y)], &RED));

        chart
            .draw_series(LineSeries::new(
                tomorrow_temps.iter().enumerate().map(|(i, &temp)| (i as i32, temp as i32)),
                &BLUE,
            ))
            .unwrap()
            .label("Mañana")
            .legend(|(x, y)| PathElement::new([(x, y), (x + 20, y)], &BLUE));

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()
            .unwrap();

        println!("Gráfico generado: temperatura.png");
    } else {
        println!("Error: No se pudo obtener la información del clima.");
    }

    Ok(())
}

// Estructuras para deserializar la respuesta JSON
#[derive(Deserialize)]
struct WeatherResponse {
    hourly: Hourly,
}

#[derive(Deserialize)]
struct Hourly {
    time: Vec<String>,
    temperature_2m: Vec<f64>,
}