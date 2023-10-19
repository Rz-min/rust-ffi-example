//
mod filter;

use filter::BitCoinFilter;
use lib::FilterService;


#[no_mangle]
pub extern "C" fn load_plugin() -> Box<dyn FilterService> {
    Box::new(BitCoinFilter::new())
}