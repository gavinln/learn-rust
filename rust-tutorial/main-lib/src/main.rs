mod geo_map;

fn main() {
    let my_favorite_place = geo_map::get_hawaii_location();
    println!("{:?}", my_favorite_place);
}
