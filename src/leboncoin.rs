extern crate reqwest;
pub(crate) mod personal_info;
use crate::publisher::Publisher;
pub struct Leboncoin;

mod parser;
mod request;

impl Publisher for Leboncoin {
    fn publish(&self, ad: crate::common::Ad) -> bool {
        crate::jwt_decoder::check_jwt_expiration(personal_info::LBC_TOKEN);

        let imgs_upload_response = request::upload_file(ad.imgs_path.first().unwrap());
        let imgs_lbc_ref = parser::parse_file_upload(&imgs_upload_response);
        let imgs_lbc_ref_2 = Image {
            name: imgs_lbc_ref.filename,
            url: imgs_lbc_ref.url,
        };
        let send_answer: String = request::send(ad, vec![imgs_lbc_ref_2]);
        let ad_id = parser::parse_send(&send_answer);
        let submit_answer = request::submit(ad_id).unwrap();
        let submit_ret = parser::parse_submit(&submit_answer);
        println!("submit_ret = {:#?}", submit_ret);
        true
    }
}

use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub name: String,
    pub url: String,
}