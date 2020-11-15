// use std::collections::HashSet;
use std::sync::mpsc;
use std::thread;

fn main() {
    println!("Hello, world!");
    let triangles = count_triangles(200, 200);
    println!("Finished count.");
    println!("Number of triangle permutations: {}", triangles.len());
    // for x in &triangles {
    //     println!("{:?}", x);
    // }
}

fn count_triangles_threaded(width : i16, height: i16) -> Vec<[i16; 6]> {
    let mut triangles: Vec<[i16; 6]> = Vec::new();
    let mut handles = Vec::new();
    let (sender, receiver) = mpsc::channel();

    for i in 0..width {
        for j in 0..height {
            let sender_clone = sender.clone();
            handles.push(thread::spawn(move || {
              
                for x1 in 0..width {
                    for y1 in 0..height {
                        if i != x1 || j != y1 {
                            let x2 = i + j - y1;
                            let y2 = j - i + x1;
                            if x2 >= 0 && x2 < width && y2 >= 0 && y2 < height {
                                sender_clone.send([i, j, x1, y1, x2, y2]).unwrap();
                            }
                        }
                    }
                }
                drop(sender_clone);
            }));
        }
    }

    drop(sender);

    for thread in handles {
        thread.join().expect("Child thread panicked.");
    }

    for received in receiver {
        triangles.push(received);
    }
    triangles
}

fn count_triangles(width : i16, height: i16) -> Vec<[i16; 6]> {
    let mut triangles: Vec<[i16; 6]> = Vec::new();

    for i in 0..width {
        for j in 0..height {

            for x1 in 0..width {
                for y1 in 0..height {
                    if i != x1 || j != y1 {
                        let x2 = i + j - y1;
                        let y2 = j - i + x1;
                        if x2 >= 0 && x2 < width && y2 >= 0 && y2 < height {
                            triangles.push([i, j, x1, y1, x2, y2]);
                        }
                    }
                }
            }

        }
    }
    triangles
}
