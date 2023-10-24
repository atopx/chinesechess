use std::fs::File;
use std::io::BufReader;
use std::sync::OnceLock;

pub struct Book {
    pub data: Vec<[isize; 3]>,
}

static BOOK: OnceLock<Book> = OnceLock::new();

impl Book {
    pub fn get() -> &'static Book {
        // 获取或初始化 Logger
        BOOK.get_or_init(|| {
            let mut reader = BufReader::new(File::open("book.dat").unwrap());
            Book {
                data: bincode::deserialize_from(&mut reader).unwrap(),
            }
        })
    }

    // search 二分查找法
    pub fn search(&self, vl: isize) -> Option<usize> {
        let mut low = 0;
        let mut hig = self.data.len() - 1;

        while low <= hig {
            let mid = (low + hig) >> 1;
            let value = self.data[mid][0];
            if value < vl {
                low = mid + 1
            } else if value > vl {
                hig = mid - 1
            } else {
                return Some(mid);
            }
        }
        return None;
    }
}
