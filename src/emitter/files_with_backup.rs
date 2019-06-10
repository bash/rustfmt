use super::*;
use std::fs;

pub(crate) struct FilesWithBackupEmitter;

impl FilesWithBackupEmitter {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl Emitter for FilesWithBackupEmitter {
    fn write_file(
        &mut self,
        FormattedFile {
            original_text,
            formatted_text,
            filename,
            ..
        }: FormattedFile<'_>,
    ) -> Result<bool, io::Error> {
        let filename = ensure_real_path(filename);
        if original_text != formatted_text {
            // Do a little dance to make writing safer - write to a temp file
            // rename the original to a .bk, then rename the temp file to the
            // original.
            let tmp_name = filename.with_extension("tmp");
            let bk_name = filename.with_extension("bk");

            fs::write(&tmp_name, formatted_text)?;
            fs::rename(filename, bk_name)?;
            fs::rename(tmp_name, filename)?;
        }
        Ok(false)
    }
}