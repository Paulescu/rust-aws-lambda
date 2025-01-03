use chrono::{DateTime, Datelike, Utc};
use lambda_http::tracing::{error, info};
use polars::prelude::*;
use rand::Rng;
use serde::Serialize;

/// Get the current unix timestamp in milliseconds
///
/// ## Returns
///
/// The current unix timestamp in milliseconds
pub fn get_unix_timestamp_ms() -> i64 {
    // Get current timestamp in seconds
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    now * 1000
}

#[derive(Debug, Serialize, Clone)]
pub struct Trip {
    tpep_pickup_datetime: DateTime<Utc>,
    tpep_dropoff_datetime: DateTime<Utc>,
    trip_distance: f64,
    fare_amount: f64,
}

/// Get fake trips
///
/// ## Arguments
///
/// * `from_ms` - Unix timestamp in milliseconds to filter trips after this time
/// * `n_results` - Maximum number of trips to return
///
/// ## Returns
///
/// Returns a Result containing a Vec of Trip structs if successful, or an error if the data is invalid
async fn get_fake_trips(from_ms: i64, n_results: i64) -> anyhow::Result<Vec<Trip>> {
    // Create a random number generator
    let mut rng = rand::thread_rng();

    // Create n_results fake trips
    let trips = (0..n_results)
        .map(|_| {
            let random_seconds = rng.gen_range(0..60);
            let pickup_time =
                DateTime::<Utc>::from_timestamp(from_ms / 1000 + random_seconds, 0).unwrap();
            let dropoff_time = DateTime::<Utc>::from_timestamp(
                from_ms / 1000 + random_seconds + rng.gen_range(300..3600),
                0,
            )
            .unwrap();

            Trip {
                tpep_pickup_datetime: pickup_time,
                tpep_dropoff_datetime: dropoff_time,
                trip_distance: rng.gen_range(0.5..20.0),
                fare_amount: rng.gen_range(2.5..100.0),
            }
        })
        .collect();

    Ok(trips)
}

/// Get trips from the database or fake data
///
/// ## Arguments
///
/// * `from_ms` - Unix timestamp in milliseconds to filter trips after this time
/// * `n_results` - Maximum number of trips to return
/// * `fake_data` - Whether to return fake data instead of real data
///
/// ## Returns
///
/// Returns a Result containing a Vec of Trip structs if successful, or an error if the
/// file cannot be read or the data is invalid
pub async fn get_trips(
    from_ms: i64,
    n_results: i64,
    fake_data: Option<bool>,
) -> anyhow::Result<Vec<Trip>> {
    if fake_data.unwrap_or(false) {
        return get_fake_trips(from_ms, n_results).await;
    }

    let (year, month) = get_year_and_month(from_ms);
    info!("Extracted year: {}, month: {}", year, month);

    // Download the parquet file
    info!(
        "Downloading parquet file for year: {}, month: {}",
        year, month
    );
    let file_path = download_parquet_file(year, month).await?;

    // Get the trips from the file
    let trips = get_trips_from_file(&file_path, from_ms, n_results)?;

    info!("Returning {} trips", trips.len());
    Ok(trips)
}

/// Download the parquet file from the URL
///
/// ## Arguments
///
/// * `year` - Year of the file to download
/// * `month` - Month of the file to download
///
/// ## Returns
///
/// Returns a Result containing the file path if successful, or an error if the file cannot be downloaded
async fn download_parquet_file(year: i32, month: i32) -> anyhow::Result<String> {
    let url = format!(
        "https://d37ci6vzurychx.cloudfront.net/trip-data/yellow_tripdata_{}-{:02}.parquet",
        year, month
    );
    let file_path = format!("/tmp/yellow_tripdata_{}-{:02}.parquet", year, month);

    // Check if the file already exists. If it does, return the file path.
    if tokio::fs::try_exists(&file_path).await? {
        info!("File {} already exists", &file_path);
        return Ok(file_path);
    }

    info!("Downloading file from {}", &url);
    let response = reqwest::get(&url).await?;
    if response.status().is_success() {
        let bytes = response.bytes().await?;

        // async copy of bytes to file
        tokio::fs::write(&file_path, bytes).await?;

        info!("File {} downloaded successfully", &file_path);
    } else {
        error!("Failed to download file");
    }
    Ok(file_path)
}

/// Reads taxi trip data from a parquet file and returns a vector of Trip structs
///
/// # Arguments
///
/// * `file_path` - Path to the parquet file containing taxi trip data
/// * `from_ms` - Unix timestamp in milliseconds to filter trips after this time
/// * `n_results` - Maximum number of trips to return
///
/// # Returns
///
/// Returns a Result containing a Vec of Trip structs if successful, or an error if the
/// file cannot be read or the data is invalid
fn get_trips_from_file(file_path: &str, from_ms: i64, n_results: i64) -> anyhow::Result<Vec<Trip>> {
    let df = LazyFrame::scan_parquet(file_path, Default::default())?
        .select([
            col("tpep_pickup_datetime"),
            col("tpep_dropoff_datetime"),
            col("trip_distance"),
            col("fare_amount"),
        ])
        .filter(col("tpep_pickup_datetime").gt_eq(lit(from_ms * 1_000_000)))
        .sort("tpep_pickup_datetime", Default::default())
        .limit(n_results as u32)
        .collect()?;

    let pickup_series = df
        .column("tpep_pickup_datetime")?
        .datetime()
        .expect("pickup datetime column should be datetime type");

    let dropoff_series = df
        .column("tpep_dropoff_datetime")?
        .datetime()
        .expect("dropoff datetime column should be datetime type");

    let distance_series = df
        .column("trip_distance")?
        .f64()
        .expect("distance column should be f64 type");

    let fare_series = df
        .column("fare_amount")?
        .f64()
        .expect("fare column should be f64 type");

    // Convert to Vec<Trip>
    let trips: Vec<Trip> = (0..df.height())
        .map(|i| Trip {
            tpep_pickup_datetime: DateTime::<Utc>::from_timestamp_nanos(
                pickup_series.get(i).unwrap(),
            ),
            tpep_dropoff_datetime: DateTime::<Utc>::from_timestamp_nanos(
                dropoff_series.get(i).unwrap(),
            ),
            trip_distance: distance_series.get(i).unwrap(),
            fare_amount: fare_series.get(i).unwrap(),
        })
        .collect();

    Ok(trips)
}

/// Get the year and month from a unix timestamp in milliseconds
///
/// ## Arguments
///
/// * `from_ms` - Unix timestamp in milliseconds
///
/// ## Returns
///
/// Returns a tuple containing the year and month
fn get_year_and_month(from_ms: i64) -> (i32, i32) {
    let datetime = DateTime::<Utc>::from_timestamp(from_ms / 1000, 0).unwrap();
    (datetime.year(), datetime.month() as i32)
}
