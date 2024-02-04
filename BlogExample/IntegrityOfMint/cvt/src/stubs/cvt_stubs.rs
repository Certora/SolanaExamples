extern "C" {
    fn CVT_assume_c(_c: bool);
    fn CVT_assert_c(_c: bool);
    
    fn CVT_nondet_u8_c() ->  u8; 
    fn CVT_nondet_u16_c() ->  u16; 
    fn CVT_nondet_u32_c() ->  u32; 
    fn CVT_nondet_u64_c() ->  u64;
    fn CVT_nondet_usize_c() ->  usize;
    
    fn CVT_nondet_i8_c() ->  i8;
    fn CVT_nondet_i16_c() ->  i16;
    fn CVT_nondet_i32_c() -> i32;
    fn CVT_nondet_i64_c() ->  i64;
}

#[allow(non_snake_case)]
pub fn CVT_assume_impl(c: bool) {
    unsafe { CVT_assume_c(c); }
}
#[allow(non_snake_case)]
pub fn CVT_assert_impl(c: bool) {
    unsafe { CVT_assert_c(c); }
}
#[allow(non_snake_case)]
pub fn CVT_nondet_u8_impl() ->  u8 {
    unsafe { return CVT_nondet_u8_c(); }
}
#[allow(non_snake_case)]
pub fn CVT_nondet_u16_impl() ->  u16 {
    unsafe { return CVT_nondet_u16_c(); }
}
#[allow(non_snake_case)]
pub fn CVT_nondet_u32_impl() ->  u32 {
    unsafe { return CVT_nondet_u32_c(); }
}
#[allow(non_snake_case)]
pub fn CVT_nondet_u64_impl() ->  u64 {
    unsafe { return CVT_nondet_u64_c(); }
}
#[allow(non_snake_case)]
pub fn CVT_nondet_usize_impl() ->  usize {
    unsafe { return CVT_nondet_usize_c(); }
}
#[allow(non_snake_case)]
pub fn CVT_nondet_i8_impl() ->  i8 {
    unsafe { return CVT_nondet_i8_c(); }
}
#[allow(non_snake_case)]
pub fn CVT_nondet_i16_impl() ->  i16 {
    unsafe { return CVT_nondet_i16_c(); }
}
#[allow(non_snake_case)]
pub fn CVT_nondet_i32_impl() -> i32 {
    unsafe { return CVT_nondet_i32_c(); } 
}
#[allow(non_snake_case)]
pub fn CVT_nondet_i64_impl() ->  i64 {
    unsafe { return CVT_nondet_i64_c(); }
}


macro_rules! CVT_nondet_array_of_bytes {
    ($name_impl:ident, $name_c: ident, $size:expr) => {
        #[allow(improper_ctypes)]
        #[allow(non_snake_case)]
	    extern "C" {
	        fn $name_c() -> [u8; $size];
	    }
  	    #[allow(non_snake_case)]
	    pub fn $name_impl() -> [u8; $size] {
	        unsafe {
		        return $name_c();
	        }
	    }
    }
}


CVT_nondet_array_of_bytes!(CVT_nondet_array_of_32_bytes_impl,
                           CVT_nondet_array_of_32_bytes_c,
                           32);



