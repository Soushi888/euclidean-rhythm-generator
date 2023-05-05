use std::vec;

fn reverse_matrix(matrix: &mut Vec<Vec<u8>>) {
    let max_row_len = matrix.iter().map(|row| row.len()).max().unwrap_or(0);

    let reversed_matrix: Vec<Vec<u8>> = (0..max_row_len)
        .map(|i| {
            matrix
                .iter()
                .filter_map(|row| row.get(i))
                .cloned()
                .collect()
        })
        .collect();

    *matrix = reversed_matrix;
}

fn euclidean_rhythm(beats: usize, steps: usize) -> Vec<u8> {
    let mut rests = steps - beats;
    let first_line = vec![1; std::cmp::min(beats, rests)];
    let mut matrix = vec![first_line];

    if beats < rests {
        while rests > 0 {
            if rests >= beats {
                matrix.push(vec![0; beats]);
                rests -= beats;
            } else {
                matrix.push(vec![0; rests]);
                rests = 0;
            }
        }
    } else {
        let mut beats = beats;
        while beats > 0 {
            let previous_line_value = matrix.last().unwrap().iter().sum::<u8>();
            if previous_line_value > 0 {
                matrix.push(vec![0; rests]);
                beats -= rests;
            } else {
                if beats >= rests {
                    matrix.push(vec![1; rests]);
                    beats -= rests;
                } else {
                    matrix.push(vec![1; beats]);
                    beats = 0;
                }
            }
        }
    }

    reverse_matrix(matrix.as_mut());

    matrix.concat()
}

pub fn e(k: usize, n: usize) -> Vec<u8> {
    euclidean_rhythm(k, n)
}

pub fn inverse_matrix(matrix: &mut Vec<u8>) -> Vec<u8> {
    matrix.iter_mut().for_each(|beat| {
        if *beat == 0 {
            *beat = 1;
        } else {
            *beat = 0;
        }
    });

    matrix.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[ignore]
    #[test]
    fn test_tressio_e_3_8() {
        let expected = vec![1, 0, 0, 1, 0, 0, 1, 0];
        let actual = e(3, 8);

        assert_eq!(expected, actual);
    }

    // #[ignore]
    #[test]
    fn test_quintillo_e_5_8() {
        let expected = vec![1, 0, 1, 1, 0, 1, 1, 0];
        let actual = e(5, 8);

        assert_eq!(expected, actual);
    }

    // #[ignore]
    #[test]
    fn test_venda_e_5_12() {
        let expected = vec![1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0];
        let actual = e(5, 12);

        assert_eq!(expected, actual);
    }

    // #[ignore]
    #[test]
    fn test_bembe_e_7_12() {
        let expected = vec![1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0];
        let actual = e(7, 12);

        assert_eq!(expected, actual);
    }

    // #[ignore]
    #[test]
    fn test_e_5_16() {
        let expected = vec![1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0];
        let actual = e(5, 16);

        assert_eq!(expected, actual);
    }

    // #[ignore]
    #[test]
    fn test_e_7_16() {
        let expected = vec![1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0];
        let actual = e(7, 16);

        assert_eq!(expected, actual);
    }
}
