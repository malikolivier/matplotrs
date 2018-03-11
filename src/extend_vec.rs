use std::cmp::Ordering;

trait MinMaxWith<T>: IntoIterator<Item = T> {
    fn min_with<F>(&self, f: F) -> Option<&T>
    where
        F: Fn(&T, &T) -> Ordering;
    fn max_with<F>(&self, f: F) -> Option<&T>
    where
        F: Fn(&T, &T) -> Ordering,
    {
        self.min_with(|x1, x2| f(x1, x2).reverse())
    }
}

impl<T> MinMaxWith<T> for Vec<T> {
    fn min_with<F>(&self, f: F) -> Option<&T>
    where
        F: Fn(&T, &T) -> Ordering,
    {
        if self.is_empty() {
            None
        } else {
            let vec = self.as_slice();
            let mut min = &vec[0];
            for item in vec.iter().skip(1) {
                if let Ordering::Less = f(item, &min) {
                    min = item;
                }
            }
            Some(min)
        }
    }
}

pub fn tuple_partial_cmp_x(&(x1, _y1): &(f64, f64), &(x2, _y2): &(f64, f64)) -> Ordering {
    x1.partial_cmp(&x2).unwrap_or(Ordering::Less)
}

pub fn tuple_partial_cmp_y(&(_x1, y1): &(f64, f64), &(_x2, y2): &(f64, f64)) -> Ordering {
    y1.partial_cmp(&y2).unwrap_or(Ordering::Less)
}

pub trait HasMinMax<T>
where
    T: PartialOrd,
{
    fn min_max_with<F>(&self, f: F) -> Option<(&T, &T)>
    where
        F: Fn(&T, &T) -> Ordering;

    fn min_max(&self) -> Option<(&T, &T)> {
        self.min_max_with(|v1: &T, v2: &T| {
            v1.partial_cmp(&v2).unwrap_or(Ordering::Less)
        })
    }
}

impl<T> HasMinMax<T> for Vec<Vec<T>>
where
    T: PartialOrd,
{
    fn min_max_with<F>(&self, f: F) -> Option<(&T, &T)>
    where
        F: Fn(&T, &T) -> Ordering,
    {
        let mut min = None;
        let mut max = None;
        for single_series in self {
            single_series.min_with(&f).map(|min_candidate| {
                if min.is_none() || (min.is_some() && min_candidate < min.unwrap()) {
                    min = Some(min_candidate);
                }
            });
            single_series.max_with(&f).map(|max_candidate| {
                if max.is_none() || (max.is_some() && max_candidate > max.unwrap()) {
                    max = Some(max_candidate);
                }
            });
        }
        match (min, max) {
            (Some(min), Some(max)) => Some((min, max)),
            _ => None,
        }
    }
}
