use cpal::*;
fn main() {
    let default_in = default_input_device().expect("Error finding defalt input");
    let default_out = default_output_device().expect("Error finding defalt output");
    let event_loop = EventLoop::new();

    let in_format = default_in.default_input_format().expect("Error getting default input format");
    let out_format = default_out.default_output_format().expect("Error getting default output format");

    let input = event_loop.build_input_stream(&default_in, &in_format).expect("Error building input stream");
    let output = event_loop.build_output_stream(&default_out, &out_format).expect("Failed to build output stream");
    event_loop.play_stream(input.to_owned());
    //event_loop.play_stream(output);

    event_loop.run(move |stream_id, stream_data| {
        if stream_id == input {
             match stream_data {
                 StreamData::Input { buffer: UnknownTypeInputBuffer::U16(mut buffer) } => {
                     println!("Input buffer size: {}", buffer.len());
                     for elem in buffer.iter() {
                         //println!("input {:?}", elem)
                     }
                     //println!();
                 },
                 StreamData::Input { buffer: UnknownTypeInputBuffer::I16(mut buffer) } => {
                     println!("Input buffer size: {}", buffer.len());
                     for elem in buffer.iter() {
                         //println!("input {:?}", elem)
                     }
                     //println!();
                 },
                 StreamData::Input { buffer: UnknownTypeInputBuffer::F32(mut buffer) } => {
                     println!("Input buffer size: {}", buffer.len());
                     for elem in buffer.iter() {
                         //println!("input {:?}", elem)
                     }
                     //println!();
                 },
                 _ => (),
             };

        } else {
             match stream_data {
                 StreamData::Output { buffer: UnknownTypeOutputBuffer::U16(mut buffer) } => {
                     //println!("Output buffer size: {}", buffer.len());
                     for elem in buffer.iter_mut() {
                         //println!("output {:?}", elem)
                         //*elem = 0;
                         //println!("output u16");
                     }
                     //println!();
                 },
                 StreamData::Output { buffer: UnknownTypeOutputBuffer::I16(mut buffer) } => {
                     //println!("Output buffer size: {}", buffer.len());
                     for elem in buffer.iter_mut() {
                         //println!("output {:?}", elem)
                         //*elem = 0;
                         //println!("output I16");
                     }
                     //println!();
                 },
                 StreamData::Output { buffer: UnknownTypeOutputBuffer::F32(mut buffer) } => {
                     //println!("Output buffer size: {}", buffer.len());
                     for elem in buffer.iter_mut() {
                        //println!("output {:?}", elem)
                        //*elem = 0.0;
                        //println!("output F32 {}", elem);
                     }
                     //println!();
                 },
                 _ => (),
             };
        }
    });
}
