use configuration_schema::validator::NoSchema;
use configuration_schema::yaml::load_from_file;

fn main() {
    for filename in std::env::args().skip(1) {
        let v = load_from_file(filename, &NoSchema::new());
        println!("{:#?}", v);
    }
}
