use crate::TimeoutFuture;
use crate::audio::*;
use crate::bootstrap_image;

/// Plays a set of special effects on a certain "number"  
/// All numbers that have a special effect:  
/// - 1
/// - 21
/// - 42
/// - 67
/// - 69
/// - 420
/// - 666
/// - 777
/// - 1000
/// - 12345
/// - 9000
/// - 80085
pub async fn special_effects(count: u32) {
    // WARN: this code is made in yandere dev style
    match count {
        1 => {
            AUDIO_1_1.with(|a| {
                let _ = a.play();
            });
            TimeoutFuture::new(2000).await;
            AUDIO_1_2.with(|a| {
                let _ = a.play();
            });
        }
        21 => AUDIO_21.with(|a| {
            let _ = a.play();
        }),
        42 => AUDIO_42.with(|a| {
            let _ = a.play();
        }),
        67 => {
            AUDIO_67.with(|a| {
                let _ = a.play();
            });
            let _ = bootstrap_image(
                "https://i.kym-cdn.com/photos/images/newsfeed/003/128/463/b28",
                Some("the 67 kid with the blue lasers coming out of his eyes".to_string()),
                3000,
                Some("half".to_string()),
            );
        }
        69 => AUDIO_69.with(|a| {
            let _ = a.play();
        }),
        420 => AUDIO_420.with(|a| {
            let _ = a.play();
        }),
        666 => AUDIO_666.with(|a| {
            let _ = a.play();
        }),
        777 => {
            let _ = bootstrap_image(
                "https://media1.tenor.com/m/WUWygJ0Fwz8AAAAd/jago33-slot-machine.gif",
                Some("slot machine go BRRRRRRRRRRRR".to_string()),
                6000,
                Some("slotmachine".to_string()),
            );

            AUDIO_777.with(|a| {
                let _ = a.play();
            });
        }
        1000 => AUDIO_1000.with(|a| {
            let _ = a.play();
            // TODO: add xbox gif at fixed bottom
        }),
        12345 => AUDIO_12345.with(|a| {
            let _ = a.play();
        }),
        9000 => AUDIO_9000.with(|a| {
            let _ = a.play();
        }),
        80085 => AUDIO_80085.with(|a| {
            let _ = a.play();
        }),

        _ => {}
    }
}
