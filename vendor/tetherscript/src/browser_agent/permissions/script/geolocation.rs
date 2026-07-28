//! Geolocation bridge state serialization.

use super::quote;
use crate::browser_agent::permissions::GeolocationEmulation;

const UNAVAILABLE: &str = "{kind:'error',code:2,message:'position unavailable'}";

pub(super) fn object(geolocation: &GeolocationEmulation) -> String {
    match geolocation {
        GeolocationEmulation::Position(pos) => format!(
            "{{kind:'position',coords:{{latitude:{},longitude:{},accuracy:{}}},timestamp:0}}",
            pos.latitude, pos.longitude, pos.accuracy
        ),
        GeolocationEmulation::Error(error) => format!(
            "{{kind:'error',code:{},message:{}}}",
            error.code.number(),
            quote(&error.message)
        ),
        GeolocationEmulation::Unavailable => UNAVAILABLE.into(),
    }
}
