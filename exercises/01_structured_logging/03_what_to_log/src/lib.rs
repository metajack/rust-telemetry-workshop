mod logger;

pub use logger::TestLogger;

use std::time::Instant;

/// Given a list of order numbers, compute the total price.
///
/// # Exercise
///
/// Add log statements to `get_total` and `get_order_details`, our two units of work, to capture
/// the data points we discussed:
/// - the start and end of each unit of work
/// - the duration of each unit of work
/// - the outcome of each unit of work
///
/// Refer to the test files for the expected output format.
pub fn get_total(order_numbers: &[u64]) -> Result<u64, anyhow::Error> {
    let mut sum = 0;
    log::info!("START - process total price");
    let total_price_time = Instant::now();
    for on in order_numbers {
        log::info!("START - retrieve order");
        let retrieve_order_time = Instant::now();
        let order = get_order_details(*on);
        let order = match order {
            Ok(order) => {
                log::info!("END - retrieve order - SUCCESS - {}ms", retrieve_order_time.elapsed().as_millis());
                order
            },
            Err(e) => {
                log::info!("END - retrieve order - ERROR - {}ms", retrieve_order_time.elapsed().as_millis());
                log::info!("END - process total price - ERROR - {}ms", total_price_time.elapsed().as_millis());
                return Err(e);
            },
        };
        sum += order.price;
    }
    log::info!("END - process total price - SUCCESS - {}ms", total_price_time.elapsed().as_millis());

    Ok(sum)
}

pub struct OrderDetails {
    pub order_number: u64,
    pub price: u64,
}

/// A dummy function to simulate what would normally be a database query.
fn get_order_details(order_number: u64) -> Result<OrderDetails, anyhow::Error> {
    if order_number % 4 == 0 {
        Err(anyhow::anyhow!("Failed to talk to the database"))
    } else {
        let prices = vec![999, 1089, 1029];
        Ok(OrderDetails {
            order_number,
            price: prices[order_number as usize % prices.len()],
        })
    }
}
