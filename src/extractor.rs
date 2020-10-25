use chrono::{DateTime, SecondsFormat, TimeZone, Utc};
use exif::In;
use exif::Tag;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::os::macos::fs::MetadataExt;
use std::time::{SystemTime, UNIX_EPOCH};
use std::path::Path;
use crate::error::ImagextractorError;
use std::io::Write;

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Clone)]
pub struct ImageInfo {
    filename: String,
    size: u64,
    // #[serde(with = "date_time_format")]
    // created_time: DateTime<Utc>,
    // #[serde(with = "date_time_format")]
    // modified_time: DateTime<Utc>,
    created_time: String,
    modified_time: String,
    orientation: String,
    capture_time: String,
    camera_model: String,
    camera_serial: String,
}

const TAGS: [Tag; 4] = [
    Tag::Orientation,
    Tag::Model,
    Tag::DateTimeOriginal,
    Tag::BodySerialNumber,
];

fn generate_output_file(path: &str) -> String {
    let index = path.rfind('.').unwrap();
    let (file_name, _) = path.split_at(index);
    let mut output_file = file_name.to_string();
    output_file.push_str(".json");
    output_file
}

pub fn extract_img_info(path: &str) {
    let file = File::open(path).unwrap();

    let output_file = generate_output_file(path.clone());

    let mut image_info: ImageInfo = Default::default();

    let metadata = file.metadata().unwrap();
    let c_time = metadata.created().unwrap();
    let m_time = metadata.modified().unwrap();
    let created_time = system_time_to_date_time(c_time);
    let modified_time = system_time_to_date_time(m_time);

    image_info.filename = path.to_string();
    image_info.created_time = created_time.to_rfc3339_opts(SecondsFormat::Nanos, true);
    image_info.modified_time = modified_time.to_rfc3339_opts(SecondsFormat::Nanos, true);
    image_info.size = metadata.st_size();

    let mut buf_reader = std::io::BufReader::new(&file);
    let exif_reader = exif::Reader::new();
    let exif = exif_reader.read_from_container(&mut buf_reader).unwrap();

    // for f in exif.fields() {
    //     println!(
    //         "{} - {} - {}",
    //         f.tag,
    //         f.ifd_num,
    //         f.display_value().with_unit(&exif)
    //     );
    // }

    for tag in TAGS.iter() {
        match exif.get_field(*tag, In::PRIMARY) {
            Some(field) => {
                println!(
                    " {} - {} - {}",
                    tag,
                    field.ifd_num,
                    field.display_value().with_unit(&exif)
                );

                let chars_to_trim: &[char] = &[' ', '"'];
                let field_value = field
                    .display_value()
                    .to_string()
                    .trim_matches(chars_to_trim)
                    .to_string();
                match *tag {
                    Tag::Orientation => {
                        image_info.orientation = field_value;
                    }
                    Tag::Model => {
                        image_info.camera_model = field_value;
                    }
                    Tag::DateTimeOriginal => {
                        image_info.capture_time = field_value;
                    }
                    Tag::BodySerialNumber => {
                        image_info.camera_serial = field_value;
                    }
                    _ => println!("{}", tag),
                }
            }
            None => (),
        }
    }
    let data = serde_json::to_string_pretty(&image_info).unwrap();
    // println!("{}", data);
    write_to_file(data, output_file).unwrap();
}

fn write_to_file(data: String, file_path: String) -> Result<(), ImagextractorError> {
    let path = Path::new(file_path.as_str());
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut out_file = match File::create(&path) {
        Err(why) => {
            eprintln!("couldn't create {}: {}", display, why);
            return Err(ImagextractorError::IOError(why));
        },
        Ok(out_file) => out_file,
    };

    match out_file.write_all(data.as_bytes()) {
        Err(why) => {
            eprintln!("couldn't write to {}: {}", display, why);
            Err(ImagextractorError::IOError(why))
        },
        Ok(_) =>  {
            println!("successfully wrote to {}", display);
            Ok(())
        },
    }
}

fn extract_file(image_name: String, image_info: ImageInfo) {}

fn system_time_to_date_time(t: SystemTime) -> DateTime<Utc> {
    let (sec, nsec) = match t.duration_since(UNIX_EPOCH) {
        Ok(dur) => (dur.as_secs() as i64, dur.subsec_nanos()),
        Err(e) => {
            // unlikely but should be handled
            let dur = e.duration();
            let (sec, nsec) = (dur.as_secs() as i64, dur.subsec_nanos());
            if nsec == 0 {
                (-sec, 0)
            } else {
                (-sec - 1, 1_000_000_000 - nsec)
            }
        }
    };
    Utc.timestamp(sec, nsec)
}

fn system_to_string(t: SystemTime) -> String {
    system_time_to_date_time(t).to_rfc3339_opts(SecondsFormat::Nanos, true)
}

#[allow(dead_code)]
mod date_time_format {
    use chrono::{DateTime, Utc};
    use serde::de::Visitor;
    use serde::{de, Deserializer, Serializer};
    use std::fmt;

    struct DateTimeVisitor;

    impl<'de> Visitor<'de> for DateTimeVisitor {
        type Value = DateTime<Utc>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an valid UTC date time string")
        }

        fn visit_str<E>(self, value: &str) -> Result<DateTime<Utc>, E>
        where
            E: de::Error,
        {
            Ok(value.parse::<DateTime<Utc>>().unwrap())
        }
    }

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let data = date.to_string();
        serializer.serialize_str(data.as_str())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(DateTimeVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{FixedOffset, SecondsFormat, Utc};
    use std::error::Error;
    use std::fs;
    use std::fs::File;
    use std::os::macos::fs::MetadataExt;
    use std::path::Path;

    #[test]
    fn datetime_parse() {
        let dt = Utc.ymd(2020, 8, 13).and_hms_nano(10, 57, 6, 773358405);

        let parse_dt = "2020-08-13T10:57:06.773358405Z"
            .parse::<DateTime<Utc>>()
            .unwrap();

        assert_eq!(dt, parse_dt);
        assert_eq!(
            "2020-08-13T10:57:06.773358405Z",
            dt.to_rfc3339_opts(SecondsFormat::Nanos, true)
        )
    }

    #[test]
    fn image_serialization() {
        let expect_img = r#"{
  "filename": "CAM18839.jpg",
  "size": 1164980,
  "created_time": "2020-08-13T10:57:06.773358405Z",
  "modified_time": "2020-08-13T10:57:06.773358405Z",
  "orientation": "1",
  "capture_time": "2020-08-09T12:58:32",
  "camera_model": "EOS 5D Mark IV",
  "camera_serial": "025021000535"
}"#;

        let dt = Utc.ymd(2020, 8, 13).and_hms_nano(10, 57, 6, 773358405);

        let img = ImageInfo {
            filename: "CAM18839.jpg".to_string(),
            size: 1164980,
            created_time: dt.to_rfc3339_opts(SecondsFormat::Nanos, true),
            modified_time: dt.to_rfc3339_opts(SecondsFormat::Nanos, true),
            orientation: "1".to_string(),
            capture_time: "2020-08-09T12:58:32".to_string(),
            camera_model: "EOS 5D Mark IV".to_string(),
            camera_serial: "025021000535".to_string(),
        };

        let serialized_img = serde_json::to_string_pretty(&img).unwrap();
        // dbg!(serialized_img);
        assert_eq!(serialized_img, expect_img);

        let parsed_img: ImageInfo = serde_json::from_str(expect_img).unwrap();
        // dbg!(parsed_img);
        assert_eq!(parsed_img, img.clone())
    }
}
