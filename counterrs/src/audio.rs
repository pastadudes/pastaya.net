// MORE OPTIMZATION!!!
thread_local! {
    pub static AUDIO_1_1: web_sys::HtmlAudioElement =
         web_sys::HtmlAudioElement::new_with_src("/assets/audio/1-part1.wav").unwrap();

    pub static AUDIO_1_2: web_sys::HtmlAudioElement =
         web_sys::HtmlAudioElement::new_with_src("/assets/audio/1-part2.wav").unwrap();

    pub static AUDIO_21: web_sys::HtmlAudioElement =
         web_sys::HtmlAudioElement::new_with_src("https://www.myinstants.com/media/sounds/whats-9-plus-10_i5PRvD4.mp3")
             .unwrap();

    pub static AUDIO_42: web_sys::HtmlAudioElement =
         web_sys::HtmlAudioElement::new_with_src("/assets/audio/42.wav").unwrap();

    pub static AUDIO_67: web_sys::HtmlAudioElement =
         web_sys::HtmlAudioElement::new_with_src("https://www.myinstants.com//media/sounds/67_SQlv2Xv.mp3")
             .unwrap();

    pub static AUDIO_69: web_sys::HtmlAudioElement =
         web_sys::HtmlAudioElement::new_with_src("https://www.myinstants.com/media/sounds/vine-boom.mp3")
             .unwrap();

    pub static AUDIO_420: web_sys::HtmlAudioElement =
     web_sys::HtmlAudioElement::new_with_src("https://www.myinstants.com/media/sounds/im-on-that-good-kush-high-quality.mp3")
         .unwrap();

    pub static AUDIO_666: web_sys::HtmlAudioElement =
     web_sys::HtmlAudioElement::new_with_src("https://www.myinstants.com/media/sounds/susto-666.mp3")
         .unwrap();

    pub static AUDIO_777: web_sys::HtmlAudioElement =
     web_sys::HtmlAudioElement::new_with_src("https://www.myinstants.com/media/sounds/slotmachine.mp3")
         .unwrap();

    pub static AUDIO_9000: web_sys::HtmlAudioElement =
     web_sys::HtmlAudioElement::new_with_src("https://www.myinstants.com/media/sounds/its_over_9000.mp3")
         .unwrap();

    pub static AUDIO_80085: web_sys::HtmlAudioElement =
        web_sys::HtmlAudioElement::new_with_src("/assets/audio/80085.wav").unwrap();


}
