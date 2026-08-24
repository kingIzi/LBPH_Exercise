use std::io::Error;

pub type Dataset = (Vec<String>, Vec<String>, Vec<String>, Vec<String>);

pub fn read_dataset(folder_path: &str) -> Result<Dataset, Error> {
    let mut dir = std::path::Path::new(folder_path);
    if !dir.exists() {
        return Err(Error::new(
            std::io::ErrorKind::NotFound,
            "folder does not exist",
        ));
    }
    let train_path = format!("./{folder_path}/train");
    let train_path = std::path::Path::new(&train_path);
    let test_path = format!("./{folder_path}/test");
    let test_path = std::path::Path::new(&test_path);
    if !train_path.exists() || !test_path.exists() {
        return Err(Error::new(
            std::io::ErrorKind::NotFound,
            "Could not find /train or /test folders",
        ));
    }
    //train path
    let (x_train, y_train) =
        std::fs::read_dir(train_path)?.fold((Vec::new(), Vec::new()), |mut acc, curr| {
            let entry = curr.unwrap();
            let p = entry.path();
            let img_path = std::path::Path::new(&p);
            let file_name = entry.file_name();
            acc.0.push(img_path.display().to_string());
            let label = parse_label_name(file_name.into_string().unwrap().as_str());
            acc.1.push(label);
            acc
        });
    let (x_test, y_test) =
        std::fs::read_dir(test_path)?.fold((Vec::new(), Vec::new()), |mut acc, curr| {
            let entry = curr.unwrap();
            let p = entry.path();
            let img_path = std::path::Path::new(&p);
            let file_name = entry.file_name();
            acc.0.push(img_path.display().to_string());
            let label = parse_label_name(file_name.into_string().unwrap().as_str());
            acc.1.push(label);
            acc
        });
    let result = (x_train, y_train, x_test, y_test);
    Ok(result)
}

pub fn parse_label_name(file_name: &str) -> String {
    if let Some(underscore) = file_name.rfind("_") {
        return file_name[0..underscore].into();
    } else if let Some(dot) = file_name.rfind(".") {
        return file_name[0..dot].into();
    }
    file_name.into()
}
