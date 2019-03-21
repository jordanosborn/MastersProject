use std::collections::VecDeque;
use rayon::prelude::*;
use arrayfire::Array;

pub fn difference(
    arr1: &arrayfire::Array<crate::RawFtType>,
    arr2: &arrayfire::Array<crate::RawFtType>,
) -> arrayfire::Array<crate::RawType> {
    let abs = arrayfire::abs(&(arr1 - arr2));
    arrayfire::mul(&abs, &abs, true)
}

pub fn radial_average(
    arr: &[arrayfire::Array<crate::RawType>],
    annuli: &[(f32, arrayfire::Array<crate::RawType>)]
) -> Vec<Vec<(f32, f32)>> {
    //TODO: Finish this function! should return 1D array I(q) for each tau
    let mut vector = Vec::with_capacity(arr.len());
    arr.iter().enumerate().for_each(|(i, a)| {
        let average = annuli.par_iter().map(|(q, annulus)| {
                (*q, arrayfire::sum_all(&(annulus * a)).0 as f32)
            }).collect::<Vec<(f32, f32)>>();
        vector.push(
            average
        );
        println!("Radial averaged tau = {}!", i + 1);
    });
    println!("Radial averaged all time steps!");
    vector
}

pub struct Data<T: arrayfire::HasAfEnum> {
    pub time_delta: f64,
    pub data: VecDeque<Array<T>>,
    pub capacity: Option<usize>,
}

impl<T: arrayfire::HasAfEnum> Data<T> {
    pub fn new(fps: usize, capacity: Option<usize>) -> Data<T> {
        if let Some(size) = capacity {
            Data {
                time_delta: 1f64 / (fps as f64),
                data: VecDeque::with_capacity(size),
                capacity: Some(size),
            }
        } else {
            Data {
                time_delta: 1f64 / (fps as f64),
                data: VecDeque::new(),
                capacity: None,
            }
        }
    }
    pub fn push(&mut self, array: Array<T>) {
        if let Some(capacity) = self.capacity {
            if self.data.len() == capacity {
                self.data.pop_front();
            }
        }
        self.data.push_back(array);
    }
}

pub fn mean_image(
    arr: &VecDeque<arrayfire::Array<crate::RawType>>,
) -> Option<arrayfire::Array<crate::RawType>> {
    if arr.is_empty() {
        let dims = arr[0].dims();
        Some(
            arr.iter().fold(
                arrayfire::Array::new_empty(dims),
                |acc: Array<crate::RawType>, x| acc + x,
            ) / arr.len() as f32,
        )
    } else {
        None
    }
}

//Algorithm tested in python!
fn create_annulus(dimension: u64, radius: u64, thickness: u64) -> arrayfire::Array<crate::RawType> {
    let radius2 = radius * radius;
    let radius_plus_dr2 = (radius + thickness) * (radius + thickness);
    let annulus : Vec<f32> = (0..(dimension * dimension)).into_par_iter().map(|i| {
        let x = i % dimension;
        let y = i / dimension;
        let r2 = (x - dimension / 2) * (x - dimension / 2) + (y - dimension / 2) * (y - dimension / 2);
        if  radius2 <= r2 && r2 <= radius_plus_dr2 {
            1.0
        } else {
            0.0
        }
    }).collect();
    let arr = Array::new(annulus.as_slice(), arrayfire::Dim4::new(&[dimension, dimension, 1, 1]));
    let divisor = arrayfire::sum_all(&arr).0 as f32;
    arr / divisor
}

pub fn generate_annuli(dimension: Option<i64>, spacing: u64) -> Vec<(f32, arrayfire::Array<crate::RawType>)> {
    let dimension = dimension.unwrap() as u64;
    let max = (dimension / 2) as usize;
    let it = (1..max).step_by(spacing as usize).collect::<Vec<usize>>();
    it.par_iter().map(|&r| {
        ((2 * r + spacing as usize) as f32 / 2.0f32, create_annulus(dimension, r as u64, spacing))
    }).collect::<Vec<(f32, arrayfire::Array<crate::RawType>)>>()
}
