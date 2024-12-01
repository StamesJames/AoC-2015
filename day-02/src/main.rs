use serde::Deserialize;

fn main() {
    let first = first();
    println!("First Solution: {first}");
    let second = second();
    println!("Second Solution: {second}");

}

#[derive(Deserialize)]
struct PresentBox {
    l: i32,
    w: i32,
    h: i32,
}
impl PresentBox {
    fn get_paper_needed(&self) -> i32 {
        let side_lw = self.l * self.w;
        let side_lh = self.l * self.h;
        let side_wh = self.w * self.h;
        side_lw * 2 + side_lh * 2 + side_wh * 2 + side_lw.min(side_lh.min(side_wh))
    }
    fn get_volume(&self) -> i32 {
        self.l * self.w * self.h
    }
    fn get_ribbon_length(&self) -> i32 {
        (self.l + self.w).min(self.l + self.h).min(self.w + self.h) * 2 + self.get_volume()
    }
}
fn second() -> i32 {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'x')
        .has_headers(false)
        .from_path("input")
        .unwrap();
    let mut paper_needed = 0;
    for result in rdr.deserialize() {
        let present_box: PresentBox = result.unwrap();
        paper_needed += present_box.get_ribbon_length();
    }
    return paper_needed;
}
fn first() -> i32 {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'x')
        .has_headers(false)
        .from_path("input")
        .unwrap();
    let mut paper_needed = 0;
    for result in rdr.deserialize() {
        let present_box: PresentBox = result.unwrap();
        paper_needed += present_box.get_paper_needed();
    }
    return paper_needed;
}
