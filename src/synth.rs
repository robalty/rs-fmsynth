extern crate portaudio as pa;


use fltk::{app, button::*, frame::*, window::*, valuator::*};
use crate::operator;
use std::sync::mpsc;

const CHANNELS: i32 = 1;
const SAMPLE_RATE: f64 = 48000.0;
const FRAMES_PER_BUFFER: u32 = 512;

pub fn run() -> Result<(), pa::Error> {
    let (tx, rx): (mpsc::Sender<Msg>, mpsc::Receiver<Msg>) = mpsc::channel();
    let mut oa = operator::OpArray::new();
    oa.ops[2].a = 0.5;
    oa.ops[2].d = 0.5;
    oa.ops[2].s = 0.0;
    let mut cpy = oa.clone();
    let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {
        let mut idx = 0;
        match rx.try_recv() {
            Ok(x) => { 
                match x.meaning {
                    0 => {cpy = x.digest();}
                    1 => {
                        let temp = cpy.get_phase();
                        cpy = x.digest();
                        cpy.phase = temp;
                    }
                    _ => {}
                };
            }
            _ => {}
        };
        for _ in 0..frames {
            let to_play = cpy.get_sample() * (std::i16::MAX as f64) / 2.0;
            cpy.step();
            buffer[idx] = to_play as i16;
            idx += 1;
        };
        while match rx.try_recv() {
            Ok(_) => {true}
            _ => {false}
        }
        {}
        pa::Continue
    };
    let pa = pa::PortAudio::new().unwrap();
    let mut settings =
        pa.default_output_stream_settings(CHANNELS, 
                                          SAMPLE_RATE,FRAMES_PER_BUFFER)?;
    let mut stream = pa.open_non_blocking_stream(settings, callback).unwrap();
    stream.start();

    let app = app::App::default();
    let mut wind = Window::default()
        .with_size(500, 300)
        .center_screen()
        .with_label("Player");
    let mut freq_box = ValueInput::default() 
        .with_size(30, 30)
        .with_pos(70, 10)
        .with_label("Note");
    app::set_callback(
        &freq_box,
        Box::new(|| {
            let temp = freq_box.value();
            for i in 0..4 {
                oa.ops[i].frequency = temp;
            }
            tx.send(Msg::new(1, oa));
        }));

    let but_play = ToggleButton::default()
        .with_size(100, 40)
        .with_pos(40, 50)
        .with_label("Press to play");
    app::set_callback(
        &but_play,
        Box::new(|| {
            if !but_play.is_toggled() {
                //stream.stop();
            }
            else {
                oa.phase = 0.0;
                tx.send(Msg::new(0, oa));
                //stream.start();
            }
        }));

    let mut slider0: ValueSlider = ValueSlider::default()
        .with_label("op 0")
        .with_size(40, 150)
        .right_of(&but_play, 80);
    slider0.set_trigger(fltk::enums::CallbackTrigger::Changed);
    slider0.set_bounds(-5.0, 5.0);
    slider0.set_value(0.0);
    app::set_callback(
        &slider0,
        Box::new(|| {
            let temp = slider0.value();
            oa.ops[0].amt = temp;
            tx.send(Msg::new(1, oa));
        }));
    let mut counter0: Counter = Counter::default()
        .with_label("mult")
        .with_size(40, 30)
        .below_of(&slider0, 20);
    counter0.set_bounds(1.0, 10.0);
    counter0.set_step(1.0, 1);
    counter0.set_type(CounterType::SimpleCounter);
    counter0.set_value(1.0);
    app::set_callback(
        &counter0,
        Box::new(|| {
            let temp = counter0.value();
            oa.ops[0].freq_mult = temp;
            tx.send(Msg::new(1, oa));
        }));
    let mut slider1: ValueSlider = ValueSlider::default()
        .with_label("op 1")
        .with_size(40, 150)
        .right_of(&slider0, 20);
    slider1.set_trigger(fltk::enums::CallbackTrigger::Changed);
    slider1.set_bounds(-5.0, 5.0);
    slider1.set_value(0.0);
    app::set_callback(
        &slider1,
        Box::new(|| {
            let temp = slider1.value();
            oa.ops[1].amt = temp;
            tx.send(Msg::new(1, oa));
        }));   
    let mut counter1: Counter = Counter::default()
        .with_label("mult")
        .with_size(40, 30)
        .below_of(&slider1, 20);
    counter1.set_bounds(1.0, 10.0);
    counter1.set_step(1.0, 1);
    counter1.set_type(CounterType::SimpleCounter);
    counter1.set_value(1.0);
    app::set_callback(
        &counter1,
        Box::new(|| {
            let temp = counter1.value();
            oa.ops[1].freq_mult = temp;
            tx.send(Msg::new(1, oa));
        }));
    let mut slider2: ValueSlider = ValueSlider::default()
        .with_label("op 2")
        .with_size(40, 150)
        .right_of(&slider1, 20);
    slider2.set_trigger(fltk::enums::CallbackTrigger::Changed);
    slider2.set_bounds(-5.0, 5.0);
    slider2.set_value(0.0);
    app::set_callback(
        &slider2,
        Box::new(|| {
            let temp = slider2.value();
            oa.ops[2].amt = temp;
            tx.send(Msg::new(1, oa));
        }));  
    let mut counter2: Counter = Counter::default()
        .with_label("mult")
        .with_size(40, 30)
        .below_of(&slider2, 20);
    counter2.set_bounds(1.0, 10.0);
    counter2.set_step(1.0, 1);
    counter2.set_type(CounterType::SimpleCounter);
    counter2.set_value(1.0);
    app::set_callback(
        &counter2,
        Box::new(|| {
            let temp = counter2.value();
            oa.ops[2].freq_mult = temp;
            tx.send(Msg::new(1, oa));
        }));
    let mut slider3: ValueSlider = ValueSlider::default()
        .with_label("op 3")
        .with_size(40, 150)
        .right_of(&slider2, 20);
    slider3.set_trigger(fltk::enums::CallbackTrigger::Changed);
    slider3.set_bounds(-5.0, 5.0);
    slider3.set_value(0.0);
    app::set_callback(
        &slider3,
        Box::new(|| {
            let temp = slider3.value();
            oa.ops[3].amt = temp;
            tx.send(Msg::new(1, oa));
        }));
    let mut counter3: Counter = Counter::default()
        .with_label("mult")
        .with_size(40, 30)
        .below_of(&slider3, 20);
    counter3.set_bounds(1.0, 10.0);
    counter3.set_step(1.0, 1);
    counter3.set_type(CounterType::SimpleCounter);
    counter3.set_value(1.0);
    app::set_callback(
        &counter3,
        Box::new(|| {
            let temp = counter3.value();
            oa.ops[3].freq_mult = temp;
            tx.send(Msg::new(1, oa));
        }));
    wind.show();
    app.run().unwrap();
    Ok(())
}

struct Msg {
    pub meaning: i32,
    contents: operator::OpArray,
}

impl Msg {
    pub fn new(why: i32, to_send: operator::OpArray)->Msg {
        Msg{
            meaning: why,
            contents: to_send,
        }
    }

    pub fn digest(&self) -> operator::OpArray {
        self.contents
    }
}
