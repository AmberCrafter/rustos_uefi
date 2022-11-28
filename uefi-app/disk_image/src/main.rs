use std::{path::{Path, PathBuf}, fs::{self, File}, io::{self, Seek}};
 
fn main() {
    let mut args = std::env::args();
    let _exe_name = args.next().unwrap();
    let efi_path = PathBuf::from(args.next().expect("path to `.efi` files must be given as argument"));
    let fat_path = efi_path.with_extension("fat");
    let disk_path = fat_path.with_extension("gpt");
    create_fat_filesystem(&fat_path, &efi_path);
    create_gpt_disk(&disk_path, &fat_path)
}

fn create_fat_filesystem(fat_path: &Path, efi_path: &Path) {
    let efi_size = fs::metadata(&efi_path).unwrap().len();
    let mb = 1024*1024;
    let efi_size_roundup = ((efi_size-1)/mb + 1) * mb;

    let fat_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(&fat_path)
        .unwrap();
    fat_file.set_len(efi_size_roundup).unwrap();

    let fat_format = fatfs::FormatVolumeOptions::new();
    fatfs::format_volume(&fat_file, fat_format).unwrap();
    let filesystem = fatfs::FileSystem::new(&fat_file, fatfs::FsOptions::new()).unwrap();
    
    let root_dir = filesystem.root_dir();
    root_dir.create_dir("efi").unwrap();
    root_dir.create_dir("efi/boot").unwrap();
    let mut bootx64 = root_dir.create_file("efi/boot/bootx64.efi").unwrap();
    bootx64.truncate().unwrap();
    io::copy(&mut fs::File::open(&efi_path).unwrap(), &mut bootx64).unwrap();
}

fn create_gpt_disk(disk_path: &Path, fat_image: &Path) {
    let mut disk = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .truncate(true)
        .open(&disk_path)
        .unwrap();
    let partition_size = fs::metadata(&fat_image).unwrap().len();
    let disk_size = partition_size + 64 * 1024;
    disk.set_len(disk_size).unwrap();

    let mbr = gpt::mbr::ProtectiveMBR::with_lb_size(
        u32::try_from((disk_size/512) - 1).unwrap_or(0xFFFF_FFFF)
    );
    mbr.overwrite_lba0(&mut disk).unwrap();

    let block_size = gpt::disk::LogicalBlockSize::Lb512;
    let mut gpt = gpt::GptConfig::new()
        .writable(true)
        .initialized(false)
        .logical_block_size(block_size)
        .create_from_device(Box::new(&mut disk), None)
        .unwrap();
    gpt.update_partitions(Default::default()).unwrap();

    let partition_id = gpt
        .add_partition("boot", partition_size, gpt::partition_types::EFI, 0, None)
        .unwrap();
    let partition = gpt.partitions().get(&partition_id).unwrap();
    let start_offset = partition.bytes_start(block_size).unwrap();

    gpt.write().unwrap();

    disk.seek(io::SeekFrom::Start(start_offset)).unwrap();
    io::copy(&mut File::open(&fat_image).unwrap(), &mut disk).unwrap();

}