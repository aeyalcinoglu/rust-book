use average::AveragedCollection;

fn main() {
    // let mut averaged_col: AveragedCollection = Default::default();
    let mut averaged_col = AveragedCollection::default();
    println!("{:?}", averaged_col);

    averaged_col.add(4);
    // cannot run averagedCol.average
    println!("{}", averaged_col.get_average());
}
