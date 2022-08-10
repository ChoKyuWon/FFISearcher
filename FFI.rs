use std::ptr;

extern {
    fn write(fd:u32, buf:*const char, size:usize) -> u32;
}

struct Data{
    val: Vec<char>,
    len: usize,
}

fn main() {
    let mut data = Data{
        val: ['a','b','c', '\n'].to_vec(),
        len: 3
    };
    let x = unsafe { write(2, data.val.as_ptr(), 4) };
    data.val.push('d');
}