use chrono::{ Local, Timelike, Datelike };
use rosc::{ OscMessage, OscType };
use crate::{ config::CONFIG, log::{ print_flush, print_log, LogType } };

pub struct BuilderParams {
    pub addresses: Vec<String>,
    pub sync_toggle: Vec<bool>,
}

pub fn build(params: BuilderParams) -> Vec<OscMessage> {
    let dt = Local::now();

    if CONFIG.lock().unwrap().show_debug_log {
        let str = t!(
            "debug_on_make_message",
            timestamp = dt.format("%Y-%m-%d %H:%M:%S.%f"),
            minute = format!("{:<5}", params.sync_toggle[0]),
            hour = format!("{:<5}", params.sync_toggle[1]),
            date = format!("{:<5}", params.sync_toggle[2])
        );
        print_flush(print_log(str, LogType::SEND));
    }

    let mut messages: Vec<OscMessage> = Vec::new();

    // second
    let second = dt.second();

    messages.push(
        make_message(&params.addresses[0], vec![OscType::Float(((second as f32) / 60.0) as f32)])
    );
    messages.push(make_message(&params.addresses[1], vec![OscType::Int(second as i32)]));

    // minute
    let minute = dt.minute();
    messages.push(
        make_message(&params.addresses[2], vec![OscType::Float(((minute as f32) / 60.0) as f32)])
    );
    if params.sync_toggle[0] == true {
        messages.push(make_message(&params.addresses[3], vec![OscType::Int(minute as i32)]));
    }

    if params.sync_toggle[1] == true {
        // hour24
        let hour24 = dt.hour();
        messages.push(
            make_message(
                &params.addresses[4],
                vec![OscType::Float(((hour24 as f32) / 24.0) as f32)]
            )
        );
        messages.push(make_message(&params.addresses[5], vec![OscType::Int(hour24 as i32)]));

        //  hour12 & isPM
        let is_pm = dt.hour12();
        let hour12 = if is_pm.1 == 12 { 0 } else { is_pm.1 };

        messages.push(
            make_message(
                &params.addresses[6],
                vec![
                    OscType::Float(
                        (((hour12 as f32) / 12.0) as f32) + (minute as f32) / 60.0 / 12.0
                    )
                ]
            )
        );
        messages.push(make_message(&params.addresses[7], vec![OscType::Int(hour12 as i32)]));
        messages.push(make_message(&params.addresses[8], vec![OscType::Bool(is_pm.0 as bool)]));
    }

    if params.sync_toggle[2] == true {
        // day
        let day = dt.day();

        messages.push(make_message(&params.addresses[9], vec![OscType::Int(day as i32)]));

        // dofw
        let dofw = dt.weekday();

        messages.push(make_message(&params.addresses[10], vec![OscType::Int(dofw as i32)]));

        // month
        let month = dt.month();

        messages.push(make_message(&params.addresses[11], vec![OscType::Int(month as i32)]));

        // year
        let year = dt.year();

        messages.push(make_message(&params.addresses[12], vec![OscType::Int(year as i32)]));

        // splitted year
        messages.push(
            make_message(&params.addresses[13], vec![OscType::Int((year / 1000) as i32)])
        );
        messages.push(
            make_message(&params.addresses[14], vec![OscType::Int(((year % 1000) / 100) as i32)])
        );
        messages.push(
            make_message(&params.addresses[15], vec![OscType::Int(((year % 100) / 10) as i32)])
        );
        messages.push(make_message(&params.addresses[16], vec![OscType::Int((year % 10) as i32)]));
    }

    return messages;
}

pub fn make_message(addr: &str, args: Vec<OscType>) -> OscMessage {
    let message = OscMessage {
        addr: addr.to_string(),
        args: args,
    };
    return message;
}
