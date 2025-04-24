// hw06.rs
// Draw a Christmas tree in console
// Single configuration parameter: number of triangles

fn draw_tree(triangles: usize) {
    // Calculate total width: base width of last triangle
    let total_width = triangles * 2 + (triangles - 1) * 2;
    
    (1..=triangles).for_each(|t| {
        let height = t + 1;
        let base_width = t * 2;
        (0..height).for_each(|i| {
            let stars = 1 + 2 * i;
            let padding = (total_width - stars) / 2;
            // print padding spaces, stars, newline
            print!("{}{}\n", " ".repeat(padding), "*".repeat(stars));
        });
    });
    // Draw trunk: width 3, height 2
    let trunk_width = 3;
    let padding = (total_width - trunk_width) / 2;
    (0..2).for_each(|_| {
        print!("{}{}\n", " ".repeat(padding), "#".repeat(trunk_width));
    });
}

fn main() {
    let triangles = 5; // configure number of triangles here
    draw_tree(triangles);
}
