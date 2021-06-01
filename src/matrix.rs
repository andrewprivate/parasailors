// Copyright (c) 2016 Adam Perry <adam.n.perry@gmail.com>
//
// This software may be modified and distributed under the terms of the MIT license.  See the
// LICENSE file for details.

use std::ffi::CString;
use std::ops::Deref;

// use libc::{c_int, c_char};

use parasail_sys::{parasail_matrix, parasail_matrix_create, parasail_matrix_free,
                   parasail_matrix_lookup};

/// A substitution matrix to use when aligning DNA or protein. Can be reused in many profiles.
pub struct Matrix {
    matrix_type: MatrixType,
    internal_rep: *const parasail_matrix,
}

unsafe impl Send for Matrix {}
unsafe impl Sync for Matrix {}

impl Matrix {
    /// Either create a dynamic substitution matrix (as in `MatrixType::Identity`) or look up a statically allocated matrix (as in any of the native parasail PAM and BLOSUM matrices).
    ///
    /// # Examples
    ///
    /// ```
    /// # use parasailors::*;
    /// // create & lookup substitution matrices
    /// let identity_matrix = Matrix::new(MatrixType::Identity);
    /// let blosum62 = Matrix::new(MatrixType::Blosum62);
    /// let pam120 = Matrix::new(MatrixType::Pam120);
    /// ```
    pub fn new(matrix_type: MatrixType) -> Self {
        unsafe {
            // we can pass this pointer because it will outlive this unsafe block
            // parasail won't keep a hold of it after the lookup
            let matrix: *const parasail_matrix = match matrix_type {
                MatrixType::Identity => {
                    let alphabet = &CString::new("ARNDCQEGHILKMFPSTWYVBZX")
                                        .expect("An internal error has occurred (creating \
                                                 identity matrix). Please file an issue at \
                                                 https://github.\
                                                 com/dikaiosune/parasailors/issues with a sample \
                                                 of the code that caused this error.");
                    parasail_matrix_create(alphabet.as_ptr(), 1, 0)
                }
                MatrixType::IdentityWithPenalty => {
                    let alphabet = &CString::new("ARNDCQEGHILKMFPSTWYVBZX")
                                        .expect("An internal error has occurred (creating \
                                                 identity matrix). Please file an issue at \
                                                 https://github.\
                                                 com/dikaiosune/parasailors/issues with a sample \
                                                 of the code that caused this error.");
                    parasail_matrix_create(alphabet.as_ptr(), 1, -1)
                }
                _ => {
                    let lookup_name = match matrix_type {
                        MatrixType::Blosum100 => "blosum100",
                        MatrixType::Blosum30 => "blosum30",
                        MatrixType::Blosum35 => "blosum35",
                        MatrixType::Blosum40 => "blosum40",
                        MatrixType::Blosum45 => "blosum45",
                        MatrixType::Blosum50 => "blosum50",
                        MatrixType::Blosum55 => "blosum55",
                        MatrixType::Blosum60 => "blosum60",
                        MatrixType::Blosum62 => "blosum62",
                        MatrixType::Blosum65 => "blosum65",
                        MatrixType::Blosum70 => "blosum70",
                        MatrixType::Blosum75 => "blosum75",
                        MatrixType::Blosum80 => "blosum80",
                        MatrixType::Blosum85 => "blosum85",
                        MatrixType::Blosum90 => "blosum90",
                        MatrixType::Pam10 => "pam10",
                        MatrixType::Pam100 => "pam100",
                        MatrixType::Pam110 => "pam110",
                        MatrixType::Pam120 => "pam120",
                        MatrixType::Pam130 => "pam130",
                        MatrixType::Pam140 => "pam140",
                        MatrixType::Pam150 => "pam150",
                        MatrixType::Pam160 => "pam160",
                        MatrixType::Pam170 => "pam170",
                        MatrixType::Pam180 => "pam180",
                        MatrixType::Pam190 => "pam190",
                        MatrixType::Pam20 => "pam20",
                        MatrixType::Pam200 => "pam200",
                        MatrixType::Pam210 => "pam210",
                        MatrixType::Pam220 => "pam220",
                        MatrixType::Pam230 => "pam230",
                        MatrixType::Pam240 => "pam240",
                        MatrixType::Pam250 => "pam250",
                        MatrixType::Pam260 => "pam260",
                        MatrixType::Pam270 => "pam270",
                        MatrixType::Pam280 => "pam280",
                        MatrixType::Pam290 => "pam290",
                        MatrixType::Pam30 => "pam30",
                        MatrixType::Pam300 => "pam300",
                        MatrixType::Pam310 => "pam310",
                        MatrixType::Pam320 => "pam320",
                        MatrixType::Pam330 => "pam330",
                        MatrixType::Pam340 => "pam340",
                        MatrixType::Pam350 => "pam350",
                        MatrixType::Pam360 => "pam360",
                        MatrixType::Pam370 => "pam370",
                        MatrixType::Pam380 => "pam380",
                        MatrixType::Pam390 => "pam390",
                        MatrixType::Pam40 => "pam40",
                        MatrixType::Pam400 => "pam400",
                        MatrixType::Pam410 => "pam410",
                        MatrixType::Pam420 => "pam420",
                        MatrixType::Pam430 => "pam430",
                        MatrixType::Pam440 => "pam440",
                        MatrixType::Pam450 => "pam450",
                        MatrixType::Pam460 => "pam460",
                        MatrixType::Pam470 => "pam470",
                        MatrixType::Pam480 => "pam480",
                        MatrixType::Pam490 => "pam490",
                        MatrixType::Pam50 => "pam50",
                        MatrixType::Pam500 => "pam500",
                        MatrixType::Pam60 => "pam60",
                        MatrixType::Pam70 => "pam70",
                        MatrixType::Pam80 => "pam80",
                        MatrixType::Pam90 => "pam90",
                        _ => "",
                    };

                    let lookup = &CString::new(lookup_name)
                                      .expect("An internal error has occurred (matrix lookup \
                                               with hardcoded string name). Please file an issue \
                                               at https://github.\
                                               com/dikaiosune/parasailors/issues with a sample \
                                               of the code that caused this error.");

                    // we need a cast here because we have to store both mut and const
                    parasail_matrix_lookup(lookup.as_ptr())
                }
            };
            // it's OK to keep this pointer forever, it points to static const structs
            Matrix {
                internal_rep: matrix,
                matrix_type: matrix_type,
            }
        }
    }
}

#[doc(hidden)]
impl Deref for Matrix {
    type Target = *const parasail_matrix;

    fn deref(&self) -> &(*const parasail_matrix) {
        &self.internal_rep
    }
}

#[doc(hidden)]
impl Drop for Matrix {
    fn drop(&mut self) {
        if let MatrixType::Identity = self.matrix_type {
            unsafe { parasail_matrix_free(self.internal_rep as *mut parasail_matrix) }
        }

        if let MatrixType::IdentityWithPenalty = self.matrix_type {
            unsafe { parasail_matrix_free(self.internal_rep as *mut parasail_matrix) }
        }
    }
}

/// Denotes the type of the substitution matrix. Use Identity for simple edit-distance calculations.
pub enum MatrixType {
    /// The identity matrix awards 1 score for each direct match, and 0 score for each mismatch.
    Identity,
    /// An identity matrix which awards 1 score for each match and penalizes -1 for each mismatch.
    IdentityWithPenalty,
    /// The [BLOSUM](https://en.wikipedia.org/wiki/BLOSUM) 100 substitution matrix.
    Blosum100,
    /// The [BLOSUM](https://en.wikipedia.org/wiki/BLOSUM) 30 substitution matrix.
    Blosum30,
    /// The [BLOSUM](https://en.wikipedia.org/wiki/BLOSUM) 35 substitution matrix.
    Blosum35,
    /// The [BLOSUM](https://en.wikipedia.org/wiki/BLOSUM) 40 substitution matrix.
    Blosum40,
    /// The [BLOSUM](https://en.wikipedia.org/wiki/BLOSUM) 45 substitution matrix.
    Blosum45,
    /// The [BLOSUM](https://en.wikipedia.org/wiki/BLOSUM) 50 substitution matrix.
    Blosum50,
    /// The [BLOSUM](https://en.wikipedia.org/wiki/BLOSUM) 55 substitution matrix.
    Blosum55,
    /// The [BLOSUM](https://en.wikipedia.org/wiki/BLOSUM) 60 substitution matrix.
    Blosum60,
    /// The [BLOSUM](https://en.wikipedia.org/wiki/BLOSUM) 62 substitution matrix.
    Blosum62,
    /// The [BLOSUM](https://en.wikipedia.org/wiki/BLOSUM) 65 substitution matrix.
    Blosum65,
    /// The [BLOSUM](https://en.wikipedia.org/wiki/BLOSUM) 70 substitution matrix.
    Blosum70,
    /// The [BLOSUM](https://en.wikipedia.org/wiki/BLOSUM) 75 substitution matrix.
    Blosum75,
    /// The [BLOSUM](https://en.wikipedia.org/wiki/BLOSUM) 80 substitution matrix.
    Blosum80,
    /// The [BLOSUM](https://en.wikipedia.org/wiki/BLOSUM) 85 substitution matrix.
    Blosum85,
    /// The [BLOSUM](https://en.wikipedia.org/wiki/BLOSUM) 90 substitution matrix.
    Blosum90,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 10 substitution matrix.
    Pam10,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 100 substitution matrix.
    Pam100,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 110 substitution matrix.
    Pam110,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 120 substitution matrix.
    Pam120,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 130 substitution matrix.
    Pam130,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 140 substitution matrix.
    Pam140,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 150 substitution matrix.
    Pam150,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 160 substitution matrix.
    Pam160,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 170 substitution matrix.
    Pam170,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 180 substitution matrix.
    Pam180,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 190 substitution matrix.
    Pam190,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 20 substitution matrix.
    Pam20,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 200 substitution matrix.
    Pam200,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 210 substitution matrix.
    Pam210,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 220 substitution matrix.
    Pam220,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 230 substitution matrix.
    Pam230,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 240 substitution matrix.
    Pam240,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 250 substitution matrix.
    Pam250,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 260 substitution matrix.
    Pam260,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 270 substitution matrix.
    Pam270,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 280 substitution matrix.
    Pam280,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 290 substitution matrix.
    Pam290,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 30 substitution matrix.
    Pam30,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 300 substitution matrix.
    Pam300,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 310 substitution matrix.
    Pam310,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 320 substitution matrix.
    Pam320,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 330 substitution matrix.
    Pam330,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 340 substitution matrix.
    Pam340,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 350 substitution matrix.
    Pam350,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 360 substitution matrix.
    Pam360,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 370 substitution matrix.
    Pam370,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 380 substitution matrix.
    Pam380,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 390 substitution matrix.
    Pam390,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 40 substitution matrix.
    Pam40,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 400 substitution matrix.
    Pam400,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 410 substitution matrix.
    Pam410,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 420 substitution matrix.
    Pam420,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 430 substitution matrix.
    Pam430,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 440 substitution matrix.
    Pam440,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 450 substitution matrix.
    Pam450,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 460 substitution matrix.
    Pam460,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 470 substitution matrix.
    Pam470,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 480 substitution matrix.
    Pam480,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 490 substitution matrix.
    Pam490,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 50 substitution matrix.
    Pam50,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 500 substitution matrix.
    Pam500,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 60 substitution matrix.
    Pam60,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 70 substitution matrix.
    Pam70,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 80 substitution matrix.
    Pam80,
    /// The [PAM](https://en.wikipedia.org/wiki/Point_accepted_mutation) 90 substitution matrix.
    Pam90,
}
