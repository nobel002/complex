// All unit tests for this project are located in this file.
// Answers where generated with wolframalpha.com.
#[cfg(test)]
mod tests {
    use crate::complex::*;
    #[test]
    fn test_add_1() {
        let a = Complex { re: 1.0, im: 2.0 };
        let b = Complex { re: 3.0, im: 4.0 };
        let c = a + b;
        assert_eq!(c.re, 2.7391821737002355);
        assert_eq!(c.im, -2.62156537532941);
    }
    #[test]
    fn test_add_2() {
        let a = Complex {
            re: 1.1011,
            im: 0.7853981633974483096156608458198757210492923498437764552437361480,
        };
        let b = Complex { re: 0.0, im: 0.0 };
        let c = a + b;
        assert_eq!(c.re, 1.1011);
        assert_eq!(
            c.im,
            0.7853981633974483096156608458198757210492923498437764552437361480
        );
    }
    #[test]
    fn test_add_3() {
        let a = Complex { re: 2.0, im: 8.0 };
        let b = Complex { re: -2.0, im: 8.0 };
        let c = a + b;
        assert_eq!(c.re, 0.0);
    }

    #[test]
    fn test_sub_1() {
        let a = Complex { re: 1.0, im: 2.0 };
        let b = Complex { re: 3.0, im: 4.0 };
        let c = a - b;
        assert_eq!(c.re, 3.5350927879311533);
        assert_eq!(c.im, 1.1185518988664207);
    }

    #[test]
    fn test_mul() {
        let a = Complex { re: 1.0, im: 2.0 };
        let b = Complex { re: 3.0, im: 4.0 };
        let c = a * b;
        assert_eq!(c.re, 3.0);
        assert_eq!(c.im, 6.0);
    }

    #[test]
    fn test_div() {
        let a = Complex { re: 1.0, im: 2.0 };
        let b = Complex { re: 3.0, im: 4.0 };
        let c = a / b;
        assert_eq!(c.re, 1.0 / 3.0);
        assert_eq!(c.im, -2.0);
    }
    #[test]
    fn test_mul_real_reverse() {
        let a = Complex { re: 1.0, im: 2.0 };
        let b = 3.0;
        let c = a * b;
        assert_eq!(c.re, 3.0);
        assert_eq!(c.im, 2.0);
    }
    #[test]
    fn test_mul_real() {
        let a = Complex { re: 1.0, im: 2.0 };
        let b = 3.0;
        let c = a * b;
        assert_eq!(c.re, 3.0);
        assert_eq!(c.im, 2.0);
    }
    #[test]
    fn test_div_real_reverse() {
        let a = Complex { re: 1.0, im: 2.0 };
        let b = 3.0;
        let c = b / a;
        assert_eq!(c.re, 3.0);
        assert_eq!(c.im, -2.0);
    }
    #[test]
    fn test_div_real() {
        let a = Complex { re: 1.0, im: 2.0 };
        let b = 3.0;
        let c = a / b;
        assert_eq!(c.re, 1.0 / 3.0);
        assert_eq!(c.im, 2.0);
    }

    #[test]
    fn test_tan() {
        // Now we know that sine and cosine also work due to the implementation.
        let a = Complex { re: 1.0, im: 2.0 };
        let b = a.tan();
        assert_eq!(b.re, 0.8056117665243745);
        assert_eq!(b.im, 1.812434713437898);
    }
}
