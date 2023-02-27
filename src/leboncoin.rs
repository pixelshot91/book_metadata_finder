extern crate reqwest;
pub(crate) mod personal_info;
use crate::publisher::Publisher;
pub struct Leboncoin;

mod parser;
mod request;

impl Publisher for Leboncoin {
    fn publish(&self, ad: crate::common::Ad) -> bool {
        let send_answer: String = request::send(ad);
        let ad_id = parser::parse_send(&send_answer);
        let submit_answer = request::submit(ad_id).unwrap();
        let submit_ret = parser::parse_submit(&submit_answer);
        println!("submit_ret = {:#?}", submit_ret);
        true
    }
}
