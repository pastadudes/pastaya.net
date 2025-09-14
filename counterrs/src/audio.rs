use web_sys::HtmlAudioElement;
// MORE OPTIMZATION!!!
thread_local! {
    pub static AUDIO_1_1: HtmlAudioElement =
         HtmlAudioElement::new_with_src("/assets/audio/1-part1.wav").unwrap();

    pub static AUDIO_1_2: HtmlAudioElement =
         HtmlAudioElement::new_with_src("/assets/audio/1-part2.wav").unwrap();

    pub static AUDIO_21: HtmlAudioElement =
         HtmlAudioElement::new_with_src("https://www.myinstants.com/media/sounds/whats-9-plus-10_i5PRvD4.mp3")
             .unwrap();

    pub static AUDIO_42: HtmlAudioElement =
         HtmlAudioElement::new_with_src("/assets/audio/42.wav").unwrap();

    pub static AUDIO_67: HtmlAudioElement =
         HtmlAudioElement::new_with_src("https://www.myinstants.com//media/sounds/67_SQlv2Xv.mp3")
             .unwrap();

    pub static AUDIO_69: HtmlAudioElement =
         HtmlAudioElement::new_with_src("https://www.myinstants.com/media/sounds/vine-boom.mp3")
             .unwrap();

    pub static AUDIO_420: HtmlAudioElement =
     HtmlAudioElement::new_with_src("https://www.myinstants.com/media/sounds/im-on-that-good-kush-high-quality.mp3")
         .unwrap();

    pub static AUDIO_666: HtmlAudioElement =
     HtmlAudioElement::new_with_src("https://www.myinstants.com/media/sounds/susto-666.mp3")
         .unwrap();

    pub static AUDIO_777: HtmlAudioElement =
     HtmlAudioElement::new_with_src("https://www.myinstants.com/media/sounds/slotmachine.mp3")
         .unwrap();

    pub static AUDIO_1000: HtmlAudioElement =
     HtmlAudioElement::new_with_src("https://www.myinstants.com/media/sounds/xbox-360-achievement-sound.mp3")
         .unwrap();

    pub static AUDIO_12345: HtmlAudioElement = HtmlAudioElement::new_with_src(
        "https://www.myinstants.com/media/sounds/murray-hi-im-murray-from-sesame-street.mp3")
        .unwrap();

    pub static AUDIO_9000: HtmlAudioElement =
     HtmlAudioElement::new_with_src("https://www.myinstants.com/media/sounds/its_over_9000.mp3")
         .unwrap();

    pub static AUDIO_80085: HtmlAudioElement =
        HtmlAudioElement::new_with_src("/assets/audio/80085.wav").unwrap();
}
