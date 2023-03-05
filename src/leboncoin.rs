extern crate reqwest;
pub(crate) mod personal_info;
use crate::publisher::Publisher;
pub struct Leboncoin;

mod parser;
mod request;

use itertools::Itertools;

impl Publisher for Leboncoin {
    fn publish(&self, ad: crate::common::Ad) -> bool {
        crate::jwt_decoder::check_jwt_expiration(personal_info::LBC_TOKEN);

        let img_lbc_refs = ad
            .imgs_path
            .clone()
            .into_iter()
            .map(|img_filepath| {
                let imgs_upload_response = request::upload_file(&img_filepath);
                let imgs_lbc_ref = parser::parse_file_upload(&imgs_upload_response);
                Image {
                    name: imgs_lbc_ref.filename,
                    url: imgs_lbc_ref.url,
                }
            })
            .collect_vec();

        let send_answer: String = request::send(ad, img_lbc_refs);
        let ad_id = parser::parse_send(&send_answer);
        let submit_answer = request::submit(ad_id).unwrap();
        let submit_ret = parser::parse_submit(&submit_answer);
        println!("submit_ret = {:#?}", submit_ret);
        match submit_ret {
            parser::SubmitResult::Submitted => true,
            parser::SubmitResult::Captcha(_) => false,
        }
    }
}

use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub name: String,
    pub url: String,
}
