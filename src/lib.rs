extern {
	#[allow(non_snake_case)]
	#[no_mangle]
	fn zip_open(path:*const i8, flags:i32, errorp:*mut i32) -> *mut i8;
}

pub fn _lzip_open(path:*const i8, flags:i32, errorp:*mut i32) -> *mut i8 {
	unsafe{ zip_open(path, flags, errorp) }
}

//Suport For C

