pub mod models;
use self::models::*;
use crate::parser::VEDirectParse;

#[cfg(test)]
mod tests;

#[macro_export]
macro_rules! convert {
    ( $map: ident, $disc: path, $fun: ident) => {{
        if let Some((_key, value)) = $map.remove_entry(&$disc.to_string()) {
            if let Ok(val) = $fun(String::from_utf8_lossy(value.as_slice()).to_string()) {
                Some(val)
            } else {
                None
            }
        } else {
            None
        }
    }};
}

pub fn convert(mut map: VEDirectParse) -> VEDirectData {
    VEDirectData {
        V: convert!(map, Labels::V, convert_mv),
        VS: convert!(map, Labels::VS, convert_mv),
        VM: convert!(map, Labels::VM, convert_mv),
        DM: convert!(map, Labels::DM, convert_parse),
        VPV: convert!(map, Labels::VPV, convert_mv),
        PPV: convert!(map, Labels::PPV, convert_parse),
        I: convert!(map, Labels::I, convert_ma),
        IL: convert!(map, Labels::IL, convert_ma),
        LOAD: convert!(map, Labels::LOAD, convert_load),
        T: convert!(map, Labels::T, convert_parse),
        P: convert!(map, Labels::P, convert_parse),
        CE: convert!(map, Labels::CE, convert_mah),
        SOC: convert!(map, Labels::SOC, convert_parse),
        TTG: convert!(map, Labels::TTG, convert_minutes),
        Alarm: convert!(map, Labels::Alarm, convert_alarm),
        Relay: convert!(map, Labels::Relay, convert_relay),
        AR: convert!(map, Labels::AR, convert_alarm_reason),
        OR: convert!(map, Labels::OR, convert_off_reason),
        H1: convert!(map, Labels::H1, convert_mah),
        H2: convert!(map, Labels::H2, convert_mah),
        H3: convert!(map, Labels::H3, convert_mah),
        H4: convert!(map, Labels::H4, convert_parse),
        H5: convert!(map, Labels::H5, convert_parse),
        H6: convert!(map, Labels::H6, convert_mah),
        H7: convert!(map, Labels::H7, convert_mv),
        H8: convert!(map, Labels::H8, convert_mv),
        H9: convert!(map, Labels::H9, convert_parse),
        H10: convert!(map, Labels::H10, convert_parse),
        H11: convert!(map, Labels::H11, convert_parse),
        H12: convert!(map, Labels::H12, convert_parse),
        H13: convert!(map, Labels::H13, convert_parse),
        H14: convert!(map, Labels::H14, convert_parse),
        H15: convert!(map, Labels::H15, convert_mv),
        H16: convert!(map, Labels::H16, convert_mv),
        H17: convert!(map, Labels::H17, convert_dawh),
        H18: convert!(map, Labels::H18, convert_dawh),
        H19: convert!(map, Labels::H19, convert_dawh),
        H20: convert!(map, Labels::H20, convert_dawh),
        H21: convert!(map, Labels::H21, convert_parse),
        H22: convert!(map, Labels::H22, convert_dawh),
        H23: convert!(map, Labels::H23, convert_parse),
        ERR: convert!(map, Labels::ERR, convert_error_code),
        CS: convert!(map, Labels::CS, convert_state_of_operation),
        BMV: convert!(map, Labels::BMV, convert_none),
        FW: convert!(map, Labels::FW, convert_none),
        FWE: convert!(map, Labels::FWE, convert_none),
        PID: convert!(map, Labels::PID, convert_none),
        SER: convert!(map, Labels::SER, convert_none),
        HSDS: convert!(map, Labels::HSDS, convert_parse),
        MODE: convert!(map, Labels::MODE, convert_device_mode),
        AC_OUT_V: convert!(map, Labels::AC_OUT_V, convert_cv),
        AC_OUT_I: convert!(map, Labels::AC_OUT_I, convert_da),
        AC_OUT_S: convert!(map, Labels::AC_OUT_S, convert_parse),
        WARN: convert!(map, Labels::WARN, convert_warning_reason),
        Calc_sum: convert!(map, Labels::Calc_sum, convert_parse),
        Checksum: convert!(map, Labels::Checksum, convert_parse),
        BLE: convert!(map, Labels::BLE, convert_ble),
        CAP_BLE: convert!(map, Labels::CAP_BLE, convert_capble),
        Time: convert!(map, Labels::Time, convert_parse),
        Unknown: if !map.is_empty() {
            let mut vec = Vec::<String>::new();
            for (key, value) in map {
                vec.push(format!("{}: {}", key, String::from_utf8_lossy(value.as_slice())));
            }
            Some(vec)
        } else {
            None
        },
    }
}
