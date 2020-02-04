use std::io::prelude::*;
use std::fs::File;
use ndarray::Array2;

fn read_string() -> std::string::String {
    let mut f = File::open("Cube.vtk").expect("failed to open .vtk");
    // read into a String, so that you don't need to do the conversion.
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).expect("failed to read .vtk");

    // and more! See the other methods for more details.
    buffer    
}

fn read() {
    let reads = read_string();
    let vec = reads.split("\n").collect::<Vec<&str>>();
    let vert_string: String = vec[4].chars().skip(7).take(vec[4].find("float").unwrap().checked_sub(8usize).unwrap()).collect();
    let vert_count: i32 = vert_string.parse().unwrap();
    let poly_string = vec[(5+vert_count)as usize].split(" ").collect::<Vec<&str>>();
    let poly_count: i32 = poly_string[1].parse().unwrap();

    let mut vert_array = Array2::<i32>::zeros((vert_count as usize, 3));
    for i in 0..(vert_count as usize) {
        let str1 = vec[(5+i)as usize];
        let iter = str1.split(" ");
        let mut c = 0usize;
        for value in iter
        {
            //println!("{}",value);
            let parsed: f32 = value.trim_end().parse().unwrap();
            vert_array[[i, c]] = parsed as i32;
            c = c + 1;
        }
    }
    println!("{}", vert_array);
    
    let mut color_array = Array2::<f32>::zeros((vert_count as usize, 3));
    for i2 in 0..(vert_count as usize) {
        let str2 = vec[(8 + i2 + (vert_count as usize) +(poly_count as usize) )as usize];
        println!("{}",str2);
        let iter2 = str2.split(" ");
        let mut c2 = 0usize;
        for value2 in iter2
        {
            //println!("{}",value);
            let parsed: f32 = value2.trim_end().parse().unwrap();
            color_array[[i2, c2]] = parsed;
            c2 = c2 + 1;
        }
    }

    println!("{}", color_array);
    
}

fn main()
{
    read();
}