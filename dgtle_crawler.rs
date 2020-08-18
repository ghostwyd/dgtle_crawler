use std::thread;
use std::fmt;
use std::sync::{mpsc, Arc};
//use std::rc::Rc;
//use std::time::Duration;

pub struct UrlInfo{
    index: i32,
    url: String,
    file_path: String
} 

impl fmt::Display for UrlInfo{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "url:{} file_path:{} index:{}", self.url, self.file_path, self.index)
    }
}

fn main() {
    
    let (tx, rx): (mpsc::Sender<Arc<UrlInfo>>, mpsc::Receiver<Arc<UrlInfo>>) = mpsc::channel();

    let mut num = 0; 
    while num < 10 {
        let tx1 = mpsc::Sender::clone(&tx);
        thread::spawn(move || {
                tx1.send(Arc::new(UrlInfo{url:String::from("sfadfa"), file_path:String::from("./pics"), index:num})).unwrap();
        });
        num += 1;
    }

    for val in rx {
        println!("Got: {}", val);
    }

}
