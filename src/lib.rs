//Libzip-rust
//Copyright 2015(c) Jeron A. Lau

/// Direct calls to zip functions
pub mod lzip{
	#[link(name = "z")]
	extern "C"{
		pub fn zip_open(path:*const i8, flags:i32, errorp:*mut i32)
			-> *mut i8;
		//TODO: param1; struct zip
		pub fn zip_strerror(archive:*mut i8) -> *const i8;
		//struct zip_file *
    		//	zip_fopen(struct zip *archive, const char *fname,
    		//	zip_flags_t flags);
		pub fn zip_fopen(archive:*mut i8, fname:*const i8, flags:u32)
			-> *mut i8;
		//int
		//	zip_fread(struct zip_file *file, void *buf,
		//	zip_uint64_t nbytes);
		pub fn zip_fread(file:*mut i8, buf:*mut i8, nbytes:u64) -> i32;
		//int zip_close(struct zip *archive);
		pub fn zip_close(archive:*mut i8) -> i32;
	}
	pub fn open(path:*const i8, flags:i32, errorp:*mut i32) -> *mut i8 {
		unsafe{ zip_open(path, flags, errorp) }
	}

	pub fn strerror(archive:*mut i8) -> *const i8 {
		unsafe{ zip_strerror(archive) }
	}
	
	pub fn fopen(archive:*mut i8, fname:*const i8, flags:u32) -> *mut i8 {
		unsafe{ zip_fopen(archive, fname, flags) }
	}
	
	pub fn fread(file:*mut i8, buf:*mut i8, nbytes:u64) -> i32 {
		unsafe{ zip_fread(file, buf, nbytes) }
	}
	
	pub fn close(archive:*mut i8) -> i32 {
		unsafe{ zip_close(archive) }
	}
}

/// (TODO) Rust-esque calls to zip functions
pub mod zip{
}
