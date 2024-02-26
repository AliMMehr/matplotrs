use crate::element::{Circle, DashedPathElement, DynElement, IntoDynElement, PathElement};
use crate::style::{ShapeStyle, SizeDesc, BLACK};
use itertools::Itertools;
use num_traits::PrimInt;
use plotters_backend::DrawingBackend;
use std::cmp::Ordering;
use std::iter::repeat;
use std::marker::PhantomData;

/**
The line series object, which takes an iterator of data points in guest coordinate system
and creates appropriate lines and points with the given style.

# Example

```
use plotters::prelude::*;
let x_values = [0.0f64, 1., 2., 3., 4.];
let drawing_area = SVGBackend::new("line_series_point_size.svg", (300, 200)).into_drawing_area();
drawing_area.fill(&WHITE).unwrap();
let mut chart_builder = ChartBuilder::on(&drawing_area);
chart_builder.margin(10).set_left_and_bottom_label_area_size(20);
let mut chart_context = chart_builder.build_cartesian_2d(0.0..4.0, 0.0..3.0).unwrap();
chart_context.configure_mesh().draw().unwrap();
chart_context.draw_series(MatLineSeries::new(x_values.map(|x| (x, 0.3 * x)), BLACK)).unwrap();
chart_context.draw_series(MatLineSeries::new(x_values.map(|x| (x, 2.5 - 0.05 * x * x)), RED)
    .point_size(5)).unwrap();
chart_context.draw_series(MatLineSeries::new(x_values.map(|x| (x, 2. - 0.1 * x * x)), BLUE.filled())
    .point_size(4)).unwrap();
```

The result is a chart with three line series; two of them have their data points highlighted:

![](https://cdn.jsdelivr.net/gh/facorread/plotters-doc-data@64e0a28/apidoc/line_series_point_size.svg)
*/
pub struct MatLineSeries<DB: DrawingBackend, X, Y> {
    style: ShapeStyle,
    y: Vec<Y>,
    x: Vec<X>,
    point_size: u32,
    phantom: PhantomData<DB>,
}

// impl<DB: DrawingBackend, Coord: Clone + 'static> Iterator for MatLineSeries<DB, Coord> {
//     type Item = DynElement<'static, DB, Coord>;
//     fn next(&mut self) -> Option<Self::Item> {
//         if !self.data.is_empty() {
//             if self.point_size > 0 && self.point_idx < self.data.len() {
//                 let idx = self.point_idx;
//                 self.point_idx += 1;
//                 return Some(
//                     Circle::new(self.data[idx].clone(), self.point_size, self.style).into_dyn(),
//                 );
//             }
//             let mut data = vec![];
//             std::mem::swap(&mut self.data, &mut data);
//             Some(PathElement::new(data, self.style).into_dyn())
//         } else {
//             None
//         }
//     }
// }

impl<DB: DrawingBackend, X, Y> MatLineSeries<DB, X, Y> {
    /**
    Sets the size of the points in the series, in pixels.

    See [`MatLineSeries`] for more information and examples.
    */
    pub fn point_size(mut self, size: u32) -> Self {
        self.point_size = size;
        self
    }
}

macro_rules! impl_line_series_from_y_for_int_type {
    ($value:ty) => {
        impl<DB: DrawingBackend, Y> MatLineSeries<DB, $value, Y> {
            /**
            Creates a new line series from the given iterator for y values. The x values will be 0..y.len().
            */
            pub fn from_y<I: IntoIterator<Item = Y>>(y_iter: I) -> Self {
                let y: Vec<Y> = y_iter.into_iter().collect();

                let y_len: usize = y.len();
                let possibly_wrong_y_len: $value = y_len as $value;

                let x: Vec<$value>;

                if (possibly_wrong_y_len as usize) != y_len {
                    x = (0..=<$value>::MAX)
                        .chain(repeat(<$value>::MAX))
                        .take(y_len)
                        .collect();
                } else {
                    x = (0..possibly_wrong_y_len).collect();
                }

                Self {
                    style: BLACK.into(),
                    y,
                    x,
                    point_size: 0,
                    phantom: PhantomData,
                }
            }
        }
    };
}

impl_line_series_from_y_for_int_type!(u8);
impl_line_series_from_y_for_int_type!(u16);
impl_line_series_from_y_for_int_type!(u32);
impl_line_series_from_y_for_int_type!(u64);
impl_line_series_from_y_for_int_type!(u128);
impl_line_series_from_y_for_int_type!(usize);
impl_line_series_from_y_for_int_type!(i8);
impl_line_series_from_y_for_int_type!(i16);
impl_line_series_from_y_for_int_type!(i32);
impl_line_series_from_y_for_int_type!(i64);
impl_line_series_from_y_for_int_type!(i128);
impl_line_series_from_y_for_int_type!(isize);

impl<DB: DrawingBackend, X, Y> MatLineSeries<DB, X, Y> {
    /**
    Creates a new line series from the given iterator for y values. The x values will be 0..y.len().
    */
    pub fn from_xy<XI: IntoIterator<Item = X>, YI: IntoIterator<Item = Y>>(
        x_iter: XI,
        y_iter: YI,
    ) -> Self {
        let mut x: Vec<X> = x_iter.into_iter().collect();
        let mut y: Vec<Y> = y_iter.into_iter().collect();

        match x.len().cmp(&y.len()) {
            Ordering::Less => {
                y = y.into_iter().take(x.len()).collect();
            }
            Ordering::Greater => {
                x = x.into_iter().take(y.len()).collect();
            }
            _ => (),
        }

        Self {
            style: BLACK.into(),
            y,
            x,
            point_size: 0,
            phantom: PhantomData,
        }
    }
}

// impl<DB: DrawingBackend, Coord, Iter: IntoIterator<Item = f64>> From<Iter>
//     for MatLineSeries<DB, X, Y>
// {
//     fn from(y_values: Iter) -> Self {}
// }

// /// A dashed line series, map an iterable object to the dashed line element.
// pub struct DashedLineSeries<I: Iterator + Clone, Size: SizeDesc> {
//     points: I,
//     size: Size,
//     spacing: Size,
//     style: ShapeStyle,
// }

// impl<I: Iterator + Clone, Size: SizeDesc> DashedLineSeries<I, Size> {
//     /// Create a new line series from
//     /// - `points`: The iterator of the points
//     /// - `size`: The dash size
//     /// - `spacing`: The dash-to-dash spacing (gap size)
//     /// - `style`: The shape style
//     /// - returns the created element
//     pub fn new<I0>(points: I0, size: Size, spacing: Size, style: ShapeStyle) -> Self
//     where
//         I0: IntoIterator<IntoIter = I>,
//     {
//         Self {
//             points: points.into_iter(),
//             size,
//             spacing,
//             style,
//         }
//     }
// }

// impl<I: Iterator + Clone, Size: SizeDesc> IntoIterator for DashedLineSeries<I, Size> {
//     type Item = DashedPathElement<I, Size>;
//     type IntoIter = std::iter::Once<Self::Item>;

//     fn into_iter(self) -> Self::IntoIter {
//         std::iter::once(DashedPathElement::new(
//             self.points,
//             self.size,
//             self.spacing,
//             self.style,
//         ))
//     }
// }

#[cfg(test)]
mod test {
    use crate::prelude::*;

    #[test]
    fn test_line_series() {
        let drawing_area = create_mocked_drawing_area(200, 200, |m| {
            m.check_draw_path(|c, s, _path| {
                assert_eq!(c, RED.to_rgba());
                assert_eq!(s, 3);
                // TODO when cleanup the backend coordinate defination, then we uncomment the
                // following check
                //for i in 0..100 {
                //    assert_eq!(path[i], (i as i32 * 2, 199 - i as i32 * 2));
                //}
            });

            m.drop_check(|b| {
                assert_eq!(b.num_draw_path_call, 8);
                assert_eq!(b.draw_count, 8);
            });
        });

        let mut chart = ChartBuilder::on(&drawing_area)
            .build_cartesian_2d(0..100, 0..100)
            .expect("Build chart error");

        // chart
        //     .draw_series(MatLineSeries::new(
        //         (0..100).map(|x| (x, x)),
        //         Into::<ShapeStyle>::into(RED).stroke_width(3),
        //     ))
        //     .expect("Drawing Error");
        chart
            .draw_series(DashedLineSeries::new(
                (0..=50).map(|x| (0, x)),
                10,
                5,
                Into::<ShapeStyle>::into(RED).stroke_width(3),
            ))
            .expect("Drawing Error");
    }
}
