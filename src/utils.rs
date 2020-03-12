use crate::Result;
use std::{
    fs,
    io::{self, Read, Seek},
    path::Path,
};
use zip::ZipArchive;

#[macro_export]
macro_rules! with_progress {
    ($f:expr, $m:expr) => {{
        let p = indicatif::ProgressBar::new_spinner();
        p.set_message($m);
        p.enable_steady_tick(100);

        let result = $f;

        p.finish_and_clear();
        result
    }};
}

pub fn unzip<P: AsRef<Path>, R: Read + Seek>(zip: &mut ZipArchive<R>, dir: P) -> Result<()> {
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        let outpath = dir.as_ref().join(file.sanitized_name());

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)?;
                }
            }

            let mut outfile = fs::File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))?;
            }
        }
    }
    Ok(())
}
