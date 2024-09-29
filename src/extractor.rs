use std::error::Error;
use std::fs::{create_dir_all, read_dir, remove_dir_all, remove_file, File};
use std::io::{Cursor, copy, Read, Seek};
use std::path::PathBuf;
use bytes::Bytes;
use zip::ZipArchive;

pub struct Extractor {
    output_dir: String,
    ignore :Vec<String>
}

impl Extractor {
    pub fn new(output_dir :&str, ignore :Vec<&str>) -> Self {
        Self {
            output_dir: output_dir.to_string(),
            ignore: ignore.iter().map(|&n| n.to_string()).collect::<Vec<String>>()
        }
    }

    pub fn extract(&self, data :Bytes) -> Result<(), Box<dyn Error>> {
        let cursor = Cursor::new(data.as_ref());
        let archive = ZipArchive::new(cursor)?;
        self.remove_all()?;
        self._extract(archive)?;

        Ok(())
    }

    pub fn remove_all(&self) -> Result<(), Box<dyn Error>> {
        let paths = read_dir(&self.output_dir)?;
        for path in paths {
            let path = path?.path();
            if path.file_name().is_some_and(|n| self.ignore.contains(&n.to_string_lossy().to_string())) {
                continue
            }
            if path.exists() {
                if path.is_dir() {
                    remove_dir_all(&path)?;
                    continue
                }
                remove_file(&path)?;
            }
        }

        Ok(())
    }

    fn _extract<T :Read + Seek>(&self, mut archive :ZipArchive<T>) -> Result<(), Box<dyn Error>> {
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let out_path = match file.enclosed_name() {
                Some(path) => path,
                None => continue,
            };

            if out_path.file_name().is_some_and(|n | self.ignore.contains(&n.to_string_lossy().to_string())) {
                continue
            }
            let out_path = PathBuf::from(self.output_dir.clone()).join(out_path);

            if file.is_dir() {
                create_dir_all(&out_path)?;
            } else {
                if let Some(p) = out_path.parent() {
                    if !p.exists() {
                        create_dir_all(p)?;
                    }
                }
                let mut outfile = File::create(&out_path)?;
                copy(&mut file, &mut outfile)?;
            }

            // Get and Set permissions
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                use std::fs::Permissions;
                use std::fs;

                if let Some(mode) = file.unix_mode() {
                    fs::set_permissions(&out_path, Permissions::from_mode(mode))?;
                }
            }

        }
        Ok(())
    }
}
