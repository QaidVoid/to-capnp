use capnp::{message, serialize_packed};
use serde::{self, Deserialize};
use std::collections::HashMap;
use std::env::consts::ARCH;
use std::fs::{self, File};
use std::{env, io};

mod schema_capnp;

#[derive(Debug, Deserialize)]
struct PackageJson {
    name: String,
    bin_name: String,
    description: String,
    note: String,
    version: String,
    download_url: String,
    size: String,
    bsum: String,
    build_date: String,
    src_url: String,
    web_url: String,
    build_script: String,
    build_log: String,
    category: String,
    extra_bins: String,
    icon: String,
}

#[derive(Debug, Deserialize)]
struct PackageListJson {
    #[serde(flatten)]
    packages: HashMap<String, Vec<PackageJson>>,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: {} -i <input_file> -o <output_file>", args[0]);
        std::process::exit(1);
    }

    let input_file = match args[1].as_str() {
        "-i" => &args[2],
        _ => {
            eprintln!("Expected -i flag for input file.");
            std::process::exit(1);
        }
    };

    let output_file = match args[3].as_str() {
        "-o" => &args[4],
        _ => {
            eprintln!("Expected -o flag for output file.");
            std::process::exit(1);
        }
    };

    let file = fs::read_to_string(input_file)?;
    let package_list_json: PackageListJson = serde_json::from_str(&file).unwrap();

    let mut message = message::Builder::new_default();
    let package_list_builder = message.init_root::<schema_capnp::package_list::Builder>();
    let mut packages_builder =
        package_list_builder.init_packages(package_list_json.packages.len() as u32);

    for (i, (key, value)) in package_list_json.packages.iter().enumerate() {
        let mut entry = packages_builder.reborrow().get(i as u32);
        entry.set_key(key);
        let mut package_builder = entry.init_value(value.len() as u32);
        for (j, package) in value.iter().enumerate() {
            let mut pkg_builder = package_builder.reborrow().get(j as u32);
            pkg_builder.set_name(&package.name);
            pkg_builder.set_bin_name(&package.bin_name);
            pkg_builder.set_description(&package.description);
            pkg_builder.set_note(&package.note);
            pkg_builder.set_version(&package.version);
            pkg_builder.set_download_url(&package.download_url);
            pkg_builder.set_size(&package.size);
            pkg_builder.set_bsum(&package.bsum);
            pkg_builder.set_build_date(&package.build_date);
            pkg_builder.set_src_url(&package.src_url);
            pkg_builder.set_web_url(&package.web_url);
            pkg_builder.set_build_script(&package.build_script);
            pkg_builder.set_build_log(&package.build_log);
            pkg_builder.set_category(&package.category);
            pkg_builder.set_extra_bins(&package.extra_bins);
            pkg_builder.set_icon(&package.icon);
            let family = package
                .download_url
                .split('/')
                .rev()
                .nth(1)
                .map(|v| v.to_owned())
                .filter(|v| v != ARCH);
            if let Some(family) = family {
                pkg_builder.set_family(&family);
            }
        }
    }
    let mut output_file = File::create(output_file)?;
    serialize_packed::write_message(&mut output_file, &message).unwrap();

    Ok(())
}
