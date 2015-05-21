pub mod zip {
	extern "C" {
		fn zip_open();
	}

	pub fn open() {
		unsafe{ zip_open() }
	}
}

//Suport For C

