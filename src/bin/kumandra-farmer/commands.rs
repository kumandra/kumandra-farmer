mod bench;
mod farm;

pub(crate) use bench::bench;
pub(crate) use farm::farm;
use std::path::Path;
use std::{fs, io};
use tracing::info;

pub(crate) fn wipe<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let _ = std::fs::remove_dir_all(path.as_ref().join("object-mappings"));
    (0..)
        .map(|i| path.as_ref().join(format!("plot{i}")))
        .take_while(|path| path.is_dir())
        .try_for_each(|replica_path| {
            info!(path = ?replica_path, "Erasing plot replica");
            std::fs::remove_dir_all(replica_path)
        })?;

    // TODO: Remove this after next snapshot, this is a compatibility layer to make sure we
    //  wipe old data from disks of our users
    if let Some(base_dir) = dirs::data_local_dir() {
        let _ = std::fs::remove_dir_all(base_dir.join("kumandra"));
    }

    // TODO: Remove this after next snapshot, this is a compatibility layer to make sure we
    //  wipe old data from disks of our users
    kumandra_farmer::Plot::erase(path.as_ref())?;

    // TODO: Remove this after next snapshot, this is a compatibility layer to make sure we
    //  wipe old data from disks of our users
    let identity = path.as_ref().join("identity.bin");
    info!(path = ?identity, "Erasing identity");
    if identity.exists() {
        fs::remove_file(identity)?;
    }

    Ok(())
}
