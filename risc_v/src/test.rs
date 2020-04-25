// test.rs

use crate::syscall::syscall_fs_read;

pub fn test_block() {
    // Let's test the block driver!
    let bytes_to_read = 1024 * 14;
    let buffer = crate::kmem::kmalloc(bytes_to_read);
    println!("Started test block process, buffer is at {:p}.", buffer);
    unsafe {
        syscall_fs_read(8, 5, buffer, bytes_to_read as u32, 0);
        for i in 0..16*4 {
            print!("{:02x}  ", buffer.add(i).read());
            if (i+1) % 16 == 0 {
                println!();
            }
        }
    }
    println!();
    crate::kmem::kfree(buffer);
    println!("Test block finished");
}
