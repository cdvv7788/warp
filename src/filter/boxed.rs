use std::fmt;
use std::sync::Arc;

use futures::Future;

use ::reject::{Rejection};
use super::{FilterBase, Filter, Tuple};

/// A type representing a boxed `Filter` trait object.
///
/// The filter inside is a dynamic trait object. The purpose of this type is
/// to ease returning `Filter`s from other functions.
///
/// To create one, call `Filter::boxed` on any filter.
pub struct BoxedFilter<T: Tuple> {
    filter: Arc<Filter<
        Extract = T,
        Error = Rejection,
        Future = Box<Future<Item=T, Error=Rejection> + Send>,
    > + Send + Sync>,
}

impl<T: Tuple + Send> BoxedFilter<T> {
    pub(super) fn new<F>(filter: F) -> BoxedFilter<T>
    where
        F: Filter<
            Extract=T,
        > + Send + Sync + 'static,
        Rejection: From<F::Error>,
    {
        BoxedFilter {
            filter: Arc::new(BoxingFilter {
                filter: filter.map_err(Rejection::from),
            }),
        }
    }
}

impl<T: Tuple> Clone for BoxedFilter<T> {
    fn clone(&self) -> BoxedFilter<T> {
        BoxedFilter {
            filter: self.filter.clone(),
        }
    }
}

impl<T: Tuple> fmt::Debug for BoxedFilter<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BoxedFilter")
            .finish()
    }
}

fn _assert_send() {
    fn _assert<T: Send>() {}
    _assert::<BoxedFilter<()>>();
}

impl<T: Tuple + Send> FilterBase for BoxedFilter<T> {
    type Extract = T;
    type Error = Rejection;
    type Future = Box<Future<Item=T, Error=Rejection> + Send>;

    fn filter(&self) -> Self::Future {
        self.filter.filter()
    }
}

struct BoxingFilter<F> {
    filter: F,
}

impl<F> FilterBase for BoxingFilter<F>
where
    F: Filter,
    F::Future: Send + 'static,
{
    type Extract = F::Extract;
    type Error = F::Error;
    type Future = Box<Future<Item=Self::Extract, Error=Self::Error> + Send>;

    fn filter(&self) -> Self::Future {
        Box::new(self.filter.filter())
    }
}
