use crate::error::RbPolarsErr;
use crate::prelude::*;
use crate::{RbResult, RbSeries};

impl RbSeries {
    pub fn eq(&self, rhs: &RbSeries) -> RbResult<Self> {
        let s = self
            .series
            .borrow()
            .equal(&*rhs.series.borrow())
            .map_err(RbPolarsErr::from)?;
        Ok(Self::new(s.into_series()))
    }

    pub fn neq(&self, rhs: &RbSeries) -> RbResult<Self> {
        let s = self
            .series
            .borrow()
            .not_equal(&*rhs.series.borrow())
            .map_err(RbPolarsErr::from)?;
        Ok(Self::new(s.into_series()))
    }

    pub fn gt(&self, rhs: &RbSeries) -> RbResult<Self> {
        let s = self
            .series
            .borrow()
            .gt(&*rhs.series.borrow())
            .map_err(RbPolarsErr::from)?;
        Ok(Self::new(s.into_series()))
    }

    pub fn gt_eq(&self, rhs: &RbSeries) -> RbResult<Self> {
        let s = self
            .series
            .borrow()
            .gt_eq(&*rhs.series.borrow())
            .map_err(RbPolarsErr::from)?;
        Ok(Self::new(s.into_series()))
    }

    pub fn lt(&self, rhs: &RbSeries) -> RbResult<Self> {
        let s = self
            .series
            .borrow()
            .lt(&*rhs.series.borrow())
            .map_err(RbPolarsErr::from)?;
        Ok(Self::new(s.into_series()))
    }

    pub fn lt_eq(&self, rhs: &RbSeries) -> RbResult<Self> {
        let s = self
            .series
            .borrow()
            .lt_eq(&*rhs.series.borrow())
            .map_err(RbPolarsErr::from)?;
        Ok(Self::new(s.into_series()))
    }
}

macro_rules! impl_eq_num {
    ($name:ident, $type:ty) => {
        impl RbSeries {
            pub fn $name(&self, rhs: $type) -> RbResult<Self> {
                let s = self.series.borrow().equal(rhs).map_err(RbPolarsErr::from)?;
                Ok(RbSeries::new(s.into_series()))
            }
        }
    };
}

impl_eq_num!(eq_u8, u8);
impl_eq_num!(eq_u16, u16);
impl_eq_num!(eq_u32, u32);
impl_eq_num!(eq_u64, u64);
impl_eq_num!(eq_i8, i8);
impl_eq_num!(eq_i16, i16);
impl_eq_num!(eq_i32, i32);
impl_eq_num!(eq_i64, i64);
impl_eq_num!(eq_f32, f32);
impl_eq_num!(eq_f64, f64);

macro_rules! impl_neq_num {
    ($name:ident, $type:ty) => {
        impl RbSeries {
            pub fn $name(&self, rhs: $type) -> RbResult<Self> {
                let s = self
                    .series
                    .borrow()
                    .not_equal(rhs)
                    .map_err(RbPolarsErr::from)?;
                Ok(RbSeries::new(s.into_series()))
            }
        }
    };
}

impl_neq_num!(neq_u8, u8);
impl_neq_num!(neq_u16, u16);
impl_neq_num!(neq_u32, u32);
impl_neq_num!(neq_u64, u64);
impl_neq_num!(neq_i8, i8);
impl_neq_num!(neq_i16, i16);
impl_neq_num!(neq_i32, i32);
impl_neq_num!(neq_i64, i64);
impl_neq_num!(neq_f32, f32);
impl_neq_num!(neq_f64, f64);

macro_rules! impl_gt_num {
    ($name:ident, $type:ty) => {
        impl RbSeries {
            pub fn $name(&self, rhs: $type) -> RbResult<Self> {
                let s = self.series.borrow().gt(rhs).map_err(RbPolarsErr::from)?;
                Ok(RbSeries::new(s.into_series()))
            }
        }
    };
}

impl_gt_num!(gt_u8, u8);
impl_gt_num!(gt_u16, u16);
impl_gt_num!(gt_u32, u32);
impl_gt_num!(gt_u64, u64);
impl_gt_num!(gt_i8, i8);
impl_gt_num!(gt_i16, i16);
impl_gt_num!(gt_i32, i32);
impl_gt_num!(gt_i64, i64);
impl_gt_num!(gt_f32, f32);
impl_gt_num!(gt_f64, f64);

macro_rules! impl_gt_eq_num {
    ($name:ident, $type:ty) => {
        impl RbSeries {
            pub fn $name(&self, rhs: $type) -> RbResult<Self> {
                let s = self.series.borrow().gt_eq(rhs).map_err(RbPolarsErr::from)?;
                Ok(RbSeries::new(s.into_series()))
            }
        }
    };
}

impl_gt_eq_num!(gt_eq_u8, u8);
impl_gt_eq_num!(gt_eq_u16, u16);
impl_gt_eq_num!(gt_eq_u32, u32);
impl_gt_eq_num!(gt_eq_u64, u64);
impl_gt_eq_num!(gt_eq_i8, i8);
impl_gt_eq_num!(gt_eq_i16, i16);
impl_gt_eq_num!(gt_eq_i32, i32);
impl_gt_eq_num!(gt_eq_i64, i64);
impl_gt_eq_num!(gt_eq_f32, f32);
impl_gt_eq_num!(gt_eq_f64, f64);

macro_rules! impl_lt_num {
    ($name:ident, $type:ty) => {
        impl RbSeries {
            pub fn $name(&self, rhs: $type) -> RbResult<RbSeries> {
                let s = self.series.borrow().lt(rhs).map_err(RbPolarsErr::from)?;
                Ok(RbSeries::new(s.into_series()))
            }
        }
    };
}

impl_lt_num!(lt_u8, u8);
impl_lt_num!(lt_u16, u16);
impl_lt_num!(lt_u32, u32);
impl_lt_num!(lt_u64, u64);
impl_lt_num!(lt_i8, i8);
impl_lt_num!(lt_i16, i16);
impl_lt_num!(lt_i32, i32);
impl_lt_num!(lt_i64, i64);
impl_lt_num!(lt_f32, f32);
impl_lt_num!(lt_f64, f64);

macro_rules! impl_lt_eq_num {
    ($name:ident, $type:ty) => {
        impl RbSeries {
            pub fn $name(&self, rhs: $type) -> RbResult<Self> {
                let s = self.series.borrow().lt_eq(rhs).map_err(RbPolarsErr::from)?;
                Ok(RbSeries::new(s.into_series()))
            }
        }
    };
}

impl_lt_eq_num!(lt_eq_u8, u8);
impl_lt_eq_num!(lt_eq_u16, u16);
impl_lt_eq_num!(lt_eq_u32, u32);
impl_lt_eq_num!(lt_eq_u64, u64);
impl_lt_eq_num!(lt_eq_i8, i8);
impl_lt_eq_num!(lt_eq_i16, i16);
impl_lt_eq_num!(lt_eq_i32, i32);
impl_lt_eq_num!(lt_eq_i64, i64);
impl_lt_eq_num!(lt_eq_f32, f32);
impl_lt_eq_num!(lt_eq_f64, f64);

impl RbSeries {
    pub fn eq_str(&self, rhs: String) -> RbResult<Self> {
        let s = self
            .series
            .borrow()
            .equal(rhs.as_str())
            .map_err(RbPolarsErr::from)?;
        Ok(RbSeries::new(s.into_series()))
    }

    pub fn neq_str(&self, rhs: String) -> RbResult<Self> {
        let s = self
            .series
            .borrow()
            .not_equal(rhs.as_str())
            .map_err(RbPolarsErr::from)?;
        Ok(RbSeries::new(s.into_series()))
    }

    pub fn gt_str(&self, rhs: String) -> RbResult<Self> {
        let s = self
            .series
            .borrow()
            .gt(rhs.as_str())
            .map_err(RbPolarsErr::from)?;
        Ok(RbSeries::new(s.into_series()))
    }

    pub fn gt_eq_str(&self, rhs: String) -> RbResult<Self> {
        let s = self
            .series
            .borrow()
            .gt_eq(rhs.as_str())
            .map_err(RbPolarsErr::from)?;
        Ok(RbSeries::new(s.into_series()))
    }

    pub fn lt_str(&self, rhs: String) -> RbResult<Self> {
        let s = self
            .series
            .borrow()
            .lt(rhs.as_str())
            .map_err(RbPolarsErr::from)?;
        Ok(RbSeries::new(s.into_series()))
    }

    pub fn lt_eq_str(&self, rhs: String) -> RbResult<Self> {
        let s = self
            .series
            .borrow()
            .lt_eq(rhs.as_str())
            .map_err(RbPolarsErr::from)?;
        Ok(RbSeries::new(s.into_series()))
    }
}
