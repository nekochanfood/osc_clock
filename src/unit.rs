use chrono::{ Local, Timelike, Datelike };
use rosc::OscType;
use serde::{ Serialize, Deserialize };

use crate::message::SyncFlag;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnitType {
    #[serde(rename = "second_int")]
    SecondInt,
    #[serde(rename = "second_float")]
    SecondFloat,

    #[serde(rename = "minute_int")]
    MinuteInt,
    #[serde(rename = "minute_float")]
    MinuteFloat,
    #[serde(rename = "minute_float_mixed")]
    MinuteFloatMixed,

    #[serde(rename = "hour24_int")]
    Hour24Int,
    #[serde(rename = "hour24_float")]
    Hour24Float,
    #[serde(rename = "hour24_float_mixed")]
    Hour24FloatMixed,

    #[serde(rename = "hour12_int")]
    Hour12Int,
    #[serde(rename = "hour12_float")]
    Hour12Float,
    #[serde(rename = "hour12_float_mixed")]
    Hour12FloatMixed,

    #[serde(rename = "is_pm")]
    IsPm,

    #[serde(rename = "day_int")]
    DayInt,
    #[serde(rename = "day_float")]
    DayFloat,
    #[serde(rename = "day_float_mixed")]
    DayFloatMixed,

    #[serde(rename = "day_of_week_int")]
    DayOfWeekInt,
    #[serde(rename = "day_of_week_float")]
    DayOfWeekFloat,
    #[serde(rename = "day_of_week_float_mixed")]
    DayOfWeekFloatMixed,

    #[serde(rename = "month_int")]
    MonthInt,
    #[serde(rename = "month_float")]
    MonthFloat,
    #[serde(rename = "month_float_mixed")]
    MonthFloatMixed,

    #[serde(rename = "year")]
    Year,

    #[serde(rename = "year_0")]
    Year0,
    #[serde(rename = "year_1")]
    Year1,
    #[serde(rename = "year_2")]
    Year2,
    #[serde(rename = "year_3")]
    Year3,
}

pub fn handle_unit(
    unit_type: UnitType,
    dt: chrono::DateTime<Local>,
    sync_flag: SyncFlag
) -> OscType {
    let is_minute_unit = matches!(
        unit_type,
        UnitType::MinuteInt | UnitType::MinuteFloat | UnitType::MinuteFloatMixed
    );
    let is_hour_unit = matches!(
        unit_type,
        UnitType::Hour24Int |
            UnitType::Hour24Float |
            UnitType::Hour24FloatMixed |
            UnitType::Hour12Int |
            UnitType::Hour12Float |
            UnitType::Hour12FloatMixed |
            UnitType::IsPm
    );
    let is_day_unit = matches!(
        unit_type,
        UnitType::DayInt |
            UnitType::DayFloat |
            UnitType::DayFloatMixed |
            UnitType::DayOfWeekInt |
            UnitType::DayOfWeekFloat |
            UnitType::DayOfWeekFloatMixed |
            UnitType::IsPm |
            UnitType::MonthInt |
            UnitType::MonthFloat |
            UnitType::MonthFloatMixed |
            UnitType::Year |
            UnitType::Year0 |
            UnitType::Year1 |
            UnitType::Year2 |
            UnitType::Year3
    );

    if
        ((!sync_flag.contains(SyncFlag::MINUTE)) && is_minute_unit) ||
        ((!sync_flag.contains(SyncFlag::HOUR)) && is_hour_unit) ||
        (!(sync_flag.contains(SyncFlag::DAY)) && is_day_unit)
    {
        return OscType::Nil;
    }
    println!("unit_type: {:?}", unit_type);
    match unit_type {
        UnitType::SecondInt => {
            return OscType::Int(dt.second() as i32);
        }
        UnitType::SecondFloat => {
            return OscType::Float(((dt.second() as f32) / 60.0) as f32);
        }
        UnitType::MinuteInt => {
            return OscType::Int(dt.minute() as i32);
        }
        UnitType::MinuteFloat => {
            return OscType::Float(((dt.minute() as f32) / 60.0) as f32);
        }
        UnitType::MinuteFloatMixed => {
            return OscType::Float(
                (((dt.minute() as f32) + (dt.second() as f32) / 60.0) / 60.0) as f32
            );
        }
        UnitType::Hour24Int => {
            return OscType::Int(dt.hour() as i32);
        }
        UnitType::Hour24Float => {
            return OscType::Float(((dt.hour() as f32) / 24.0) as f32);
        }
        UnitType::Hour24FloatMixed => {
            return OscType::Float(
                (((dt.hour() as f32) +
                    (dt.minute() as f32) / 60.0 +
                    (dt.second() as f32) / 3600.0) /
                    24.0) as f32
            );
        }
        UnitType::Hour12Int => {
            return OscType::Int((if dt.hour() % 12 == 0 { 12 } else { dt.hour() % 12 }) as i32);
        }
        UnitType::Hour12Float => {
            return OscType::Float(
                ((if dt.hour() % 12 == 0 { 12 } else { dt.hour() % 12 }) as f32) / 12.0
            );
        }
        UnitType::Hour12FloatMixed => {
            return OscType::Float(
                (((if dt.hour() % 12 == 0 { 12 } else { dt.hour() % 12 }) as f32) +
                    (dt.minute() as f32) / 60.0 +
                    (dt.second() as f32) / 3600.0) /
                    12.0
            );
        }
        UnitType::IsPm => {
            return OscType::Bool(dt.hour() >= 12);
        }
        UnitType::DayInt => {
            return OscType::Int(dt.day() as i32);
        }
        UnitType::DayFloat => {
            let max_days = dt
                .with_day(1)
                .unwrap()
                .with_month((dt.month() % 12) + 1)
                .unwrap_or_else(||
                    dt
                        .with_year(dt.year() + 1)
                        .unwrap()
                        .with_month(1)
                        .unwrap()
                )
                .signed_duration_since(dt.with_day(1).unwrap())
                .num_days() as f32;
            return OscType::Float((dt.day() as f32) / max_days);
        }
        UnitType::DayFloatMixed => {
            let max_days = dt
                .with_day(1)
                .unwrap()
                .with_month((dt.month() % 12) + 1)
                .unwrap_or_else(||
                    dt
                        .with_year(dt.year() + 1)
                        .unwrap()
                        .with_month(1)
                        .unwrap()
                )
                .signed_duration_since(dt.with_day(1).unwrap())
                .num_days() as f32;
            let day_fraction =
                (dt.day() as f32) -
                1.0 +
                (dt.hour() as f32) / 24.0 +
                (dt.minute() as f32) / 1440.0 +
                (dt.second() as f32) / 86400.0;
            return OscType::Float(day_fraction / max_days);
        }
        UnitType::DayOfWeekInt => {
            return OscType::Int(dt.weekday().num_days_from_monday() as i32);
        }
        UnitType::DayOfWeekFloat => {
            return OscType::Float((dt.weekday().num_days_from_monday() as f32) / 7.0);
        }
        UnitType::DayOfWeekFloatMixed => {
            let day_fraction =
                (dt.weekday().num_days_from_monday() as f32) +
                (dt.hour() as f32) / 24.0 +
                (dt.minute() as f32) / 1440.0 +
                (dt.second() as f32) / 86400.0;
            return OscType::Float(day_fraction / 7.0);
        }
        UnitType::MonthInt => {
            return OscType::Int(dt.month() as i32);
        }
        UnitType::MonthFloat => {
            return OscType::Float((dt.month() as f32) / 12.0);
        }
        UnitType::MonthFloatMixed => {
            let max_days = dt
                .with_day(1)
                .unwrap()
                .with_month((dt.month() % 12) + 1)
                .unwrap_or_else(||
                    dt
                        .with_year(dt.year() + 1)
                        .unwrap()
                        .with_month(1)
                        .unwrap()
                )
                .signed_duration_since(dt.with_day(1).unwrap())
                .num_days() as f32;
            let day_fraction =
                (dt.day() as f32) -
                1.0 +
                (dt.hour() as f32) / 24.0 +
                (dt.minute() as f32) / 1440.0 +
                (dt.second() as f32) / 86400.0;
            let month_fraction = (dt.month() as f32) - 1.0 + day_fraction / max_days;
            return OscType::Float(month_fraction / 12.0);
        }
        UnitType::Year => {
            return OscType::Int(dt.year() as i32);
        }
        UnitType::Year0 | UnitType::Year1 | UnitType::Year2 | UnitType::Year3 => {
            let year_str = dt.year().to_string();
            let index = match unit_type {
                UnitType::Year0 => 0,
                UnitType::Year1 => 1,
                UnitType::Year2 => 2,
                UnitType::Year3 => 3,
                _ => unreachable!(),
            };
            return OscType::Int(
                year_str.chars().nth(index).unwrap_or('0').to_digit(10).unwrap_or(0) as i32
            );
        }
    }
}
