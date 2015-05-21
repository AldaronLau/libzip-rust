#[cfg(feature="pkg-config")]
extern crate pkg_config;
extern crate gcc;

fn main() {
	if !build_pkgconfig() {
	  println!("cargo:rustc-flags=-l zip");
	}
	gcc::Config::new()
		.file("src/c/mkstemp.c")
		.file("src/c/zip_file_replace.c")
		.file("src/c/zip_source_deflate.c")
		.file("src/c/zip_add_entry.c")
		.file("src/c/zip_file_set_comment.c")
		.file("src/c/zip_source_error.c")
		.file("src/c/zip_close.c")
		.file("src/c/zip_file_set_external_attributes.c")
		.file("src/c/zip_source_file.c")
		.file("src/c/zip_delete.c")
		.file("src/c/zip_file_strerror.c")
		.file("src/c/zip_source_filep.c")
		.file("src/c/zip_dir_add.c")
		.file("src/c/zip_fopen.c")
		.file("src/c/zip_source_free.c")
		.file("src/c/zip_dirent.c")
		.file("src/c/zip_fopen_encrypted.c")
		.file("src/c/zip_source_function.c")
		.file("src/c/zip_discard.c")
		.file("src/c/zip_fopen_index.c")
		.file("src/c/zip_source_layered.c")
		.file("src/c/zip_entry.c")
		.file("src/c/zip_fopen_index_encrypted.c")
		.file("src/c/zip_source_open.c")
		.file("src/c/zip_error.c")
		.file("src/c/zip_fread.c")
		.file("src/c/zip_source_pkware.c")
		.file("src/c/zip_error_clear.c")
		.file("src/c/zip_get_archive_comment.c")
		.file("src/c/zip_source_pop.c")
		.file("src/c/zip_error_get.c")
		.file("src/c/zip_get_archive_flag.c")
		.file("src/c/zip_source_read.c")
		.file("src/c/zip_error_get_sys_type.c")
		.file("src/c/zip_get_compression_implementation.c")
		.file("src/c/zip_source_stat.c")
		.file("src/c/zip_error_strerror.c")
		.file("src/c/zip_get_encryption_implementation.c")
		.file("src/c/zip_source_window.c")
		.file("src/c/zip_error_to_str.c")
		.file("src/c/zip_get_name.c")
		.file("src/c/zip_source_zip.c")
		.file("src/c/zip_err_str.c")
		.file("src/c/zip_get_num_entries.c")
		.file("src/c/zip_source_zip_new.c")
		.file("src/c/zip_extra_field_api.c")
		.file("src/c/zip_memdup.c")
		.file("src/c/zip_stat.c")
		.file("src/c/zip_extra_field.c")
		.file("src/c/zip_name_locate.c")
		.file("src/c/zip_stat_index.c")
		.file("src/c/zip_fclose.c")
		.file("src/c/zip_new.c")
		.file("src/c/zip_stat_init.c")
		.file("src/c/zip_fdopen.c")
		.file("src/c/zip_open.c")
		.file("src/c/zip_strerror.c")
		.file("src/c/zip_file_add.c")
		.file("src/c/zip_set_archive_comment.c")
		.file("src/c/zip_string.c")
		.file("src/c/zip_file_error_clear.c")
		.file("src/c/zip_set_archive_flag.c")
		.file("src/c/zip_unchange_all.c")
		.file("src/c/zip_file_error_get.c")
		.file("src/c/zip_set_default_password.c")
		.file("src/c/zip_unchange_archive.c")
		.file("src/c/zip_file_get_comment.c")
		.file("src/c/zip_set_file_compression.c")
		.file("src/c/zip_unchange.c")
		.file("src/c/zip_file_get_external_attributes.c")
		.file("src/c/zip_set_name.c")
		.file("src/c/zip_unchange_data.c")
		.file("src/c/zip_file_get_offset.c")
		.file("src/c/zip_source_buffer.c")
		.file("src/c/zip_utf-8.c")
		.file("src/c/zip_filerange_crc.c")
		.file("src/c/zip_source_close.c")
		.file("src/c/zip_file_rename.c")
		.file("src/c/zip_source_crc.c")
		.include("src/c")
		.compile("libzip_rust.a");
}

#[cfg(not(feature="pkg-config"))]
fn build_pkgconfig() -> bool {
	false
}

#[cfg(feature="pkg-config")]
fn build_pkgconfig() -> bool {
	if pkg_config::find_library("zip").is_err() {
		panic!("Could not find LibZip via pkgconfig");
	}
	true
}
