use cpal::*;
use std::sync::*;
use std::thread;

//Duration to calibrate for for each parameter
const DURATION: std::time::Duration = std::time::Duration::from_secs(1);

//Find amplitude of sound leaking in to the headphones
pub fn calc_amplitude(calibrator: &Device) -> f32 {
    let event_loop = EventLoop::new();
    let buf: Arc<Mutex<Vec<f32>>> = Arc::new(Mutex::new(Vec::new()));
    //Idea for atomic bool for tracking whether a thread should continue borrowed
    // from cpal's "record_wav" example.
    //Original: https://github.com/tomaka/cpal/blob/master/examples/record_wav.rs
    let calibrating: Arc<atomic::AtomicBool> = Arc::new(atomic::AtomicBool::new(true));

    let buf_clone = buf.clone();
    let calibrating_clone = calibrating.clone();
    let format = calibrator
        .default_input_format()
        .expect("Error getting default input format");
    let input = event_loop
        .build_input_stream(&calibrator, &format)
        .expect("Error building input stream");
    //Collect samples to find amplitude
    event_loop.play_stream(input);
    thread::spawn(move || {
        event_loop.run(move |_, stream_data| {
            if !calibrating_clone.load(atomic::Ordering::Relaxed) {
                return;
            }
            //catch input in u16, i16, or f32 format
            match stream_data {
                StreamData::Input {
                    buffer: UnknownTypeInputBuffer::U16(buffer),
                } => {
                    for elem in buffer.iter() {
                        buf_clone.lock().unwrap().push(*elem as f32);
                    }
                }
                StreamData::Input {
                    buffer: UnknownTypeInputBuffer::I16(buffer),
                } => {
                    for elem in buffer.iter() {
                        buf_clone.lock().unwrap().push(*elem as f32);
                    }
                }
                StreamData::Input {
                    buffer: UnknownTypeInputBuffer::F32(buffer),
                } => {
                    for elem in buffer.iter() {
                        buf_clone.lock().unwrap().push(*elem);
                    }
                }
                _ => println!("no input"),
            };
        });
    });

    //collect samples for some time
    thread::sleep(DURATION);
    calibrating.store(false, atomic::Ordering::Relaxed);

    //find peak-to-peak amplitude of recorded sound
    let mut peak_to_peak: f32 = buf.lock().unwrap().iter().fold(
        -std::f32::INFINITY,
        |max, &item| {
            if max <= item {
                item
            } else {
                max
            }
        },
    );
    peak_to_peak -=
        buf.lock().unwrap().iter().fold(
            std::f32::INFINITY,
            |min, &item| {
                if min >= item {
                    item
                } else {
                    min
                }
            },
        );
    peak_to_peak / 2.0_f32.sqrt()
}
