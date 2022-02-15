use matrixlab::matrix::sparse::SparseMatrix;
use matrixlab::MatrixElement;
use std::fs::File;
use std::io::prelude::*;
use tokio::process::Command;

/// Applies louvains community detection algorithm to the expanded graph saved at
/// temp/expanded_coolist. Returns a Vec of interpolation matrices. Taking the
/// original adjacency matrix, A, and the interpolation matrix, P_0, then
/// A_coarse1 = P_0^t * A * P_0
pub async fn louvain(n: usize) -> Vec<SparseMatrix<usize>> {
    let _result = Command::new("../louvain/convert")
        .args(&[
            "-i",
            "temp/expanded_coolist",
            "-o",
            "temp/graph.bin",
            "-w",
            "temp/graph.weights",
        ])
        .output()
        .await
        .unwrap();

    let tree = Command::new("../louvain/community")
        .args(&["temp/graph.bin", "-l", "-1", "-w", "temp/graph.weights"])
        .output()
        .await
        .unwrap();

    let mut louvain = File::create("temp/louvain_hierarchy").unwrap();
    louvain.write_all(&tree.stdout).unwrap();

    build_interpolation_matrices(n)
}

// Helper method that constructs the interpolation matrices after Louvain's
// has created the partition tree.
fn build_interpolation_matrices(n: usize) -> Vec<SparseMatrix<usize>> {
    // number of vertices at each level
    let mut start = 0;
    let mut end = n;
    let data = std::fs::read_to_string("temp/louvain_hierarchy").unwrap();

    let tree: Vec<(usize, usize)> = data
        .trim()
        .split('\n')
        .map(|line| {
            let mut vertex = line.split_ascii_whitespace();
            let node = vertex.next().unwrap().parse().unwrap();
            let group = vertex.next().unwrap().parse().unwrap();
            (node, group)
        })
        .collect();

    let mut interpolation_matrices = vec![];

    while end <= tree.len() + 1 {
        let mut matrix_elements: Vec<MatrixElement<usize>> = vec![];
        let mut num_aggs = 0;
        let n = end - start;

        for (node, group) in tree[start..end].iter() {
            if num_aggs < *group {
                num_aggs = *group;
            }
            matrix_elements.push(MatrixElement::new(*node, *group, 1));
        }

        println!("num aggs: {}", num_aggs);
        let interpolation_matrix = SparseMatrix::new(n, num_aggs + 1, matrix_elements).unwrap();
        println!("level");
        interpolation_matrices.push(interpolation_matrix);

        start = end;
        end += num_aggs;
    }

    interpolation_matrices
}
