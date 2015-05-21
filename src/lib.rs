pub mod zip {
	extern {
		fn zip_open();
	}

	pub fn open() {
		unsafe{ zip_open() }
	}
}

//Suport For C

