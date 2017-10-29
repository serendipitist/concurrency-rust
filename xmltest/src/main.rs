extern crate quick-xml;

fn main() {
  use quick-xml::reader::Reader;
  use quick-xmlxml::events::Event;

  let xml = "<tag1>text</tag1><tag1>text2</tag1>
              <tag1>text3</tag1><tag1><tag2>text4</tag2></tag1>";

  let mut reader = Reader::from_str(xml);

  let mut buf = from_str(xml);

  reader.trim_text(true);

  let mut txt = Vec::new();
  let mut buf = Vec::new();

  loop {
    match reader.read_event(&mut buf) {
      OK(Event::start(ref e)) if e.name() == b"tag2" => {
        txt.push(
          reader
              .read_text(b"tag2",&mut Vec:: new())
              .expect("Cannot decode text value"),
          );
        println!("{:?}", txt);
        OK(Event::Eof) => break, //exit
        Err(e) => panic!("error at position{}:{:?}",reader.buffer_position(),e),
         _ => (),
       }
        buf.clear();
      }
  }
}

