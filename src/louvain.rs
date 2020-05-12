use matrixlab::matrix::sparse::SparseMatrix;
use matrixlab::MatrixElement;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

/// Applies louvains community detection algorithm to the expanded graph saved at
/// temp/expanded_coolist. Returns a Vec of interpolation matrices. Taking the
/// original adjacency matrix, A, and the interpolation matrix, P_0, then
/// A_coarse1 = P_0^t * A * P_0
pub fn louvain(n: usize) -> Vec<SparseMatrix<usize>> {
    let _result = Command::new("../louvain/convert")
        .args(&["-i", "temp/expanded_coolist", "-o", "temp/graph.bin"])
        .output()
        .unwrap();

    let tree = Command::new("../louvain/community")
        .args(&["temp/graph.bin", "-l", "-1"])
        .output()
        .unwrap();

    let mut louvain = File::create("temp/louvain_hierarchy").unwrap();
    louvain.write_all(&tree.stdout).unwrap();

    build_interpolation_matrices(n)
}

// Helper method that constructs the interpolation matrices after Louvain's
// has created the partition tree.
fn build_interpolation_matrices(n: usize) -> Vec<SparseMatrix<usize>> {
    // number of vertices at each level
    let mut n = n;
    let data = std::fs::read_to_string("temp/louvain_hierarchy").unwrap();

    let mut vertex_iter = data.split('\n').peekable();
    let mut interpolation_matrices = vec![];

    while vertex_iter.peek().is_some() {
        let mut num_aggregates = 0;
        let matrix_elements: Vec<MatrixElement<usize>> = vertex_iter
            .by_ref()
            .take(n)
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(|x| {
                let mut vertex = x.split_ascii_whitespace();
                let node = vertex.next().unwrap().parse().unwrap();
                let group = vertex.next().unwrap().parse().unwrap();
                if group > num_aggregates {
                    num_aggregates = group;
                }
                MatrixElement::new(node, group, 1)
            })
            .collect();

        let interpolation_matrix =
            SparseMatrix::new(n, num_aggregates + 1, matrix_elements).unwrap();
        interpolation_matrices.push(interpolation_matrix);
        n = num_aggregates + 1;
    }

    interpolation_matrices
}
