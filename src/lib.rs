#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::sync::{Arc, Mutex};
use std::os::raw::c_void;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct file_handle(Arc<Mutex<*mut c_void>>);

unsafe impl Send for file_handle {}
unsafe impl Sync for file_handle {}

pub mod libpmem {
    use super::*;

    pub fn pmem_file_create(filepath : &str, len: u64, mapped_len : &mut u64, is_pmem : &mut i32) -> file_handle {

        unsafe {
            let mut handle = pmem_map_file(filepath.as_ptr() as *const i8,
                                     len,
                                     (PMEM_FILE_CREATE|PMEM_FILE_EXCL) as i32,
                                     0666,
                                     mapped_len,
                                     is_pmem);

            match NonNull::new( handle) {
                Some(ptr) => file_handle( Arc::new(Mutex::new(handle))),
                None => panic!("\n Failed to create and map a pmem.")
            }
        }
    }

    pub fn pmem_file_open(filepath: &str, mapped_len: &mut u64, is_pmem: &mut i32) -> file_handle {
        println!("\n trying opening pmem file. {}", filepath);
        unsafe {
            let mut handle = pmem_map_file(filepath.as_ptr() as *const i8, 
                                     0, // Opening an existing file requires no flag(s).
                                     0, // No length as no flag is provided.
                                     0666, 
                                     mapped_len, 
                                     is_pmem);

            match NonNull::new( handle) {
                Some(ptr) => file_handle( Arc::new(Mutex::new(handle))),
                None => panic!("\n Failed to map existing pmem.")
            }
        }
    }

    pub fn pmem_file_read(filehandle: &file_handle, offset: usize, data: &[u8], len: usize) {
        println!("\n Reading from pmem, offset:{}, len:{}", offset, len);
        unsafe {
            let ret_handle = pmem_memcpy(data.as_ptr() as *mut c_void, filehandle.0.lock().unwrap().add( offset), len as u64, 0);

            match NonNull::new(ret_handle) {
                Some(ptr) => (()),
                None => panic!("\n Failed to read data from pmem.")
            }
        }
    }

    pub fn pmem_file_write(filehandle: &file_handle, offset: usize, data: &[u8], len: usize) -> Result<(), std::io::Error>{
        println!("\n Writing to pmem, offset:{}, len:{}", offset, len);
        unsafe{
            let ret_handle = pmem_memcpy_persist( filehandle.0.lock().unwrap().add( offset), data.as_ptr() as *mut c_void, len as u64);

            match NonNull::new(ret_handle) {
                Some(ptr) => Ok(()),
                None => panic!("\n Failed to read data from pmem.")
            }
        }   
    }

    pub fn pmem_file_close(filehandle: &file_handle, mapped_len: &u64) {
        unsafe {
            pmem_unmap(*filehandle.0.lock().unwrap(), *mapped_len);
        }
    }   
}

#[cfg(test)]
mod tests {
    use super::*;

    const BUFFER_SIZE: usize = 4096;
    const DEST_FILEPATH: &str = "/mnt/pmemfs0/public/testfile\0";
    const TEXT: &str = "The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog.\0"; 
    const TEXT2: &str = "hello world!";

    #[test]
    fn sample_test() {
        unsafe {
            let mut src_filehandle : i32;
            let mut buf =  vec![0; BUFFER_SIZE];

            let mut dest_filehandle;

            //let mut read_bytes;
            let mut is_pmem : i32 = 0;
            let mut mapped_len : u64 = 0;

            //src_filehandle = open( SRC_FILEPATH.as_ptr() as *const i8, O_RDONLY as i32);
            //read_bytes = read(src_filehandle, buf.as_ptr() as *mut c_void, BUFFER_SIZE as u64);
            //println!("\n Total bytes read: {}", read_bytes);
            //println!("\n First character read from the file: {}", buf[0] as char);

//            if (std::path::Path::exists( std::path::Path::new(&DEST_FILEPATH))) {
//                std::fs::remove_file(&DEST_FILEPATH);
//            }

            dest_filehandle = libpmem::pmem_file_open(&DEST_FILEPATH, &mut mapped_len, &mut is_pmem); 
            //dest_filehandle  = pmem_file_create( &DEST_FILEPATH, 4096, &mut mapped_len, &mut is_pmem);

            /*pmemaddr = pmem_map_file(fp2, 0,
				//(PMEM_FILE_CREATE|PMEM_FILE_EXCL) as i32,
                0,
				0666, &mut mapped_len, &mut is_pmem);
            */

            //pmem_memcpy_persist( pmemaddr.0.add(600), k, cc as u64);
            //libpmem::pmem_file_write(&dest_filehandle, 0, &TEXT, TEXT.chars().count());
            //libpmem::pmem_file_write(&dest_filehandle, TEXT.chars().count(), &TEXT2, TEXT2.chars().count());

            //let mut buffer = [u8; TEXT.chars().count()];
            //libpmem::pmem_file_read(&dest_filehandle, 0, &buffer, TEXT.chars().count());
            //println!("\n TEXT length: {}", TEXT.chars().count());

            //let mut buffer2 = [u8; TEXT2.chars().count()];
            //libpmem::pmem_file_read(&dest_filehandle, TEXT.chars().count(), &buffer2, TEXT2.chars().count());
            //println!("\n TEXT2 length: {}", TEXT2.chars().count());


            //close(src_filehandle);
	
            //pmem_unmap(pmemaddr.0, mapped_len);
            libpmem::pmem_file_close(&dest_filehandle, &mapped_len);

            let read_string = match std::str::from_utf8(&buffer) {
                Ok(string) => string,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };

            assert_eq!(TEXT, read_string);

            let read_string2 = match std::str::from_utf8(&buffer2) {
                Ok(string) => string,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };

            assert_eq!(TEXT2, read_string2);
        }
    }
}
