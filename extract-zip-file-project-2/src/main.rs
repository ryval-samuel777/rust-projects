extern crate zip;

use zip::ZipArchive;
use std::fs;
use std::io;
use std::process::exit;
use std::env::args;
use std::path::Path;


fn main() {
    exit(decompressed_zip());
}

fn decompressed_zip () -> i32 {

    // get argument file.zip 
    let args: Vec<_> = args().collect();

    // handing if there's no argument 
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return 1;
    }
    
    // borrow file name from argument 
    let file_name = Path::new(&*args[1]);

    // open file 
    let file = fs::File::open(&file_name).unwrap();

    // start reading file using ZipArchive function 
    let mut archive = ZipArchive::new(file).unwrap();
    
    // start get every file on archive from 0 to length of archive 
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        //setting the path where the files will be extracted
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        {
            // debug or checking log
            let comment = file.comment();
            if !comment.is_empty(){
                println!(
                    "File {} comment:{}", 
                    i,
                    comment
                )
            }
        }
        
        //the zip can contain other folders too 
        if (*file.name()).ends_with('/') {
            println!(
                "File {} extracted to \"{}\"", 
                i, 
                outpath.display()
            );
            //recursively create a new directory
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)", 
                i, 
                outpath.display(),
                file.size()
            );
            
            //if there is no parent for those files, create a new directory
            if let Some(p) = outpath.parent() {
                if !p.exists(){
                    fs::create_dir_all(&p).unwrap();
                }
            }

            let mut outfile = 
                fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap(); 
        }
        
        // set permissions for extracted zip file 
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode(){
                fs::set_permissions
                    (
                        &outpath, fs::Permissions::from_mode(mode)
                    ).unwrap();
            }
        }
    }
    0
}

