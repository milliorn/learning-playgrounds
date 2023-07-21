use csv::Reader;
use linfa::prelude::*;
use linfa::Dataset;
use linfa_trees::DecisionTree;
use ndarray::{Array, Array1, Array2};
use std::{fs::File, usize};

/* definitions */
fn get_headers(reader: &mut Reader<File>) -> Vec<String> {
    return reader
        .headers()
        .unwrap()
        .iter()
        .map(|r| r.to_owned())
        .collect();
}

fn get_records(data: &Vec<Vec<f32>>, target_index: usize) -> Array2<f32> {
    let mut records: Vec<f32> = vec![];
    for record in data.iter() {
        records.extend_from_slice(&record[0..target_index]);
    }
    return Array::from(records).into_shape((303, 13)).unwrap();
}

fn get_targets(data: &Vec<Vec<f32>>, target_index: usize) -> Array1<i32> {
    let targets = data
        .iter()
        .map(|record| record[target_index] as i32)
        .collect::<Vec<i32>>();
    return Array::from(targets);
}

fn get_data(reader: &mut Reader<File>) -> Vec<Vec<f32>> {
    return reader
        .records()
        .map(|r| {
            r.unwrap()
                .iter()
                .map(|field| field.parse::<f32>().unwrap())
                .collect::<Vec<f32>>()
        })
        .collect::<Vec<Vec<f32>>>();
}

/* reads the heart.csv file, parses its content, prepares a dataset with its contents, and returns the prepared dataset */
fn get_dataset() -> Dataset<f32, i32, ndarray::Dim<[usize; 1]>> {
    /* initialize a reader pointing to heart.csv: */
    let mut reader = Reader::from_path("./src/dataset/heart.csv").unwrap();

    /* extract the headers and data from reader: */
    let headers = get_headers(&mut reader);
    let data = get_data(&mut reader);
    /* calculate the index of target in the headers: */
    let target_index = headers.len() - 1;

    /* get the features from headers: */
    let features = headers[0..target_index].to_vec();

    /* retrieve the records and targets from data: */
    let records = get_records(&data, target_index);
    let targets = get_targets(&data, target_index);

    /* build the dataset with records, targets, and features, then return: */
    return Dataset::new(records, targets).with_feature_names(features);
}

fn main() {
    let dataset = get_dataset();
    println!("{:?}", dataset);

    /* fetch the dataset, and split into testing and training data: */
    let (train, test) = linfa_datasets::iris().split_with_ratio(0.9);

    /* initialize the model and train it with the training data: */
    let model = DecisionTree::params().fit(&train).unwrap();

    /* use the testing data to make some predictions: */
    let predictions = model.predict(&test);

    /* compare the predictions with the actual values: */
    println!("{:?}", predictions);
    println!("{:?}", test.targets);
}
