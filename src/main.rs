use std::fs::{File, OpenOptions};
use img_parts::{ImageEXIF};
use img_parts::jpeg::Jpeg;
use std::io::{BufWriter, Write, Read, BufReader, Cursor};
use png::Compression;
use std::env;
use std::path::Path;
use std::ffi::OsStr;
use std::error::Error;
use zip::write::FileOptions;

fn get_stem(p: &Path) -> String {
    p.file_stem().and_then(OsStr::to_str).or(Some("")).unwrap().to_string()
}

fn get_extension(p: &Path) -> String {
    p.extension().and_then(OsStr::to_str).or(Some("")).unwrap().to_string()
}

fn remove_png_meta(in_file: &File, out_file: &File) -> Result<(), Box<dyn Error>> {
    return remove_png_meta_stream(BufReader::new(in_file), BufWriter::new(out_file));
}

fn remove_png_meta_stream<R:Read, W: Write>(reader : BufReader<R>, writer : BufWriter<W>)-> Result<(), Box<dyn Error>> {
    let decoder = png::Decoder::new(reader);
    let (info, mut reader) = decoder.read_info()?;
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf)?;

    let mut encoder = png::Encoder::new(writer, info.width, info.height);
    encoder.set_color(info.color_type);
    encoder.set_depth(info.bit_depth);
    encoder.set_compression(Compression::Default);
    let mut writer = encoder.write_header()?;
    writer.write_image_data(&buf)?;
    Ok(())
}

fn remove_jpg_meta(in_file: &File, out_file: &File) -> Result<(), Box<dyn Error>> {
    return remove_jpg_meta_stream(BufReader::new(in_file), BufWriter::new(out_file));
}

fn remove_jpg_meta_stream<R:Read, W: Write>(mut reader: BufReader<R>, writer : BufWriter<W>) -> Result<(), Box<dyn Error>> {
    let mut raw_bytes : Vec<u8> = Vec::new();
    reader.read_to_end(&mut raw_bytes)?;
    let mut img = Jpeg::from_bytes(raw_bytes.into())?;
    img.set_exif(None);
    img.encoder().write_to(writer)?;
    Ok(())
}

fn remove_zip_meta(in_file: &File, out_file: &File) -> Result<(), Box<dyn Error>> {
    let mut in_zip = zip::ZipArchive::new(in_file)?;
    let mut out_zip = zip::ZipWriter::new(out_file);

    for i in 0..in_zip.len() {
        let mut file = in_zip.by_index(i)?;
        if file.is_dir() {
            out_zip.raw_copy_file(file)?;
            continue;
        }
        let ext_lower = get_extension(Path::new(file.name())).to_ascii_lowercase();
        if ext_lower == "png" || ext_lower == "jpg" || ext_lower == "jpeg" {
            let options = FileOptions::default()
                .last_modified_time(file.last_modified())
                .compression_method(file.compression());
            if let Some(perms) = file.unix_mode() {
                options.unix_permissions(perms);
            }
            out_zip.start_file(file.name(), options)?;

            let mut in_bytes: Vec<u8> = Vec::new();
            file.read_to_end(&mut in_bytes)?;

            let mut out_cursor = Cursor::new(Vec::new());

            let reader = BufReader::new(Cursor::new(in_bytes));
            let writer = BufWriter::new(&mut out_cursor);
            if ext_lower == "png" {
                remove_png_meta_stream(reader, writer)?;
            } else {
                remove_jpg_meta_stream(reader, writer)?;
            }

            out_zip.write_all(out_cursor.get_ref())?;
        } else {
            out_zip.raw_copy_file(file)?;
        }
    }

    out_zip.finish()?;

    Ok(())
}

fn handle_arg(arg: &String) -> Result<Option<String>, Box<dyn Error>> {
    let path = Path::new(&arg);
    let parent = path.parent();
    let stem = get_stem(path);
    let extension = get_extension(path);
    let ext_lower = extension.to_ascii_lowercase();
    let mut rm_meta_from_file: Option<fn(&File, &File) -> Result<(), Box<(dyn Error)>>> = None;
    if ext_lower == "jpg" || ext_lower == "jpeg" {
        rm_meta_from_file = Some(remove_jpg_meta);
    } else if ext_lower == "png" {
        rm_meta_from_file = Some(remove_png_meta);
    } else if ext_lower == "zip" {
        rm_meta_from_file = Some(remove_zip_meta);
    }
    if let Some(rm_meta) = rm_meta_from_file {
        let target_name = format!("{}_no_meta.{}", stem, extension);
        let result_file_path = match parent {
            None => target_name,
            Some(parent) => parent.join(target_name).to_str().unwrap().into(),
        };
        let in_file = File::open(arg)?;
        let out_file = OpenOptions::new().create_new(true).write(true).open(&result_file_path)?;
        rm_meta(&in_file, &out_file)?;
        Ok(Some(result_file_path))
    } else {
        Ok(None)
    }
}

fn main() {
    for arg in env::args().skip(1).into_iter() {
        match handle_arg(&arg) {
            Err(e) => eprintln!("ERROR   {} --> {:?}", arg, e),
            Ok(None) => {eprintln!("IGNORED {}", arg)},
            Ok(Some(result_path)) => {eprintln!("OK      {} --> {}", arg, result_path)}
        }
    }
}
