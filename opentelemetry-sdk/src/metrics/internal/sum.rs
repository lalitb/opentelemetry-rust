use std::sync::atomic::{AtomicBool, Ordering};
use std::{collections::HashMap, sync::Mutex, time::SystemTime};

use dashmap::DashMap;

use crate::attributes::AttributeSet;
use crate::metrics::data::{self, Aggregation, DataPoint, Temporality};
use opentelemetry::{global, metrics::MetricsError};

use super::{
    aggregate::{is_under_cardinality_limit, STREAM_OVERFLOW_ATTRIBUTE_SET},
    AtomicTracker, Number,
};

/// The storage for sums.
struct ValueMap<T: Number<T>> {
    //values: Mutex<HashMap<AttributeSet, T>>,
    values: DashMap<AttributeSet, T>,
    has_no_value_attribute_value: AtomicBool,
    no_attribute_value: T::AtomicTracker,
}

impl<T: Number<T>> Default for ValueMap<T> {
    fn default() -> Self {
        ValueMap::new()
    }
}

impl<T: Number<T>> ValueMap<T> {
    fn new() -> Self {
        ValueMap {
            values: DashMap::new(),
            has_no_value_attribute_value: AtomicBool::new(false),
            no_attribute_value: T::new_atomic_tracker(),
        }
    }
}

impl<T: Number<T>> ValueMap<T> {
    /*fn measure(&self, measurement: T, attrs: AttributeSet) {

        if attrs.is_empty() {
            self.no_attribute_value.add(measurement);
            self.has_no_value_attribute_value
                .store(true, Ordering::Release);
        } else if let Ok(mut values) = self.values.lock() {
            let size = values.len();
            match values.entry(attrs) {
                Entry::Occupied(mut occupied_entry) => {
                    let sum = occupied_entry.get_mut();
                    *sum += measurement;
                }
                Entry::Vacant(vacant_entry) => {
                    if is_under_cardinality_limit(size) {
                        vacant_entry.insert(measurement);
                    } else {
                        values
                            .entry(STREAM_OVERFLOW_ATTRIBUTE_SET.clone())
                            .and_modify(|val| *val += measurement)
                            .or_insert(measurement);
                        global::handle_error(MetricsError::Other("Warning: Maximum data points for metric stream exceeded. Entry added to overflow.".into()));
                    }
                }
            }
        }
    }*/
    fn measure(&self, measurement: T, attrs: AttributeSet) {
        if attrs.is_empty() {
            // Directly add measurement to the atomic tracker
            // This might need to be adjusted based on how `T::AtomicTracker` is implemented
            self.no_attribute_value.add(measurement);
            self.has_no_value_attribute_value
                .store(true, Ordering::Release);
        } else {
            let current_size = self.values.len();
            if is_under_cardinality_limit(current_size) {
                // Use entry API to update or insert measurement atomically
                self.values
                    .entry(attrs)
                    .and_modify(|e| *e += measurement)
                    .or_insert(measurement);
            } else {
                // Handle cardinality limit exceeded case
                self.values
                    .entry(STREAM_OVERFLOW_ATTRIBUTE_SET.clone()) // Assuming STREAM_OVERFLOW_ATTRIBUTE_SET is defined
                    .and_modify(|val| *val += measurement)
                    .or_insert_with(|| measurement);
                global::handle_error(MetricsError::Other("Warning: Maximum data points for metric stream exceeded. Entry added to overflow.".into()));
            }
        }
    }
}

/// Summarizes a set of measurements made as their arithmetic sum.
pub(crate) struct Sum<T: Number<T>> {
    value_map: ValueMap<T>,
    monotonic: bool,
    start: Mutex<SystemTime>,
}

impl<T: Number<T>> Sum<T> {
    /// Returns an aggregator that summarizes a set of measurements as their
    /// arithmetic sum.
    ///
    /// Each sum is scoped by attributes and the aggregation cycle the measurements
    /// were made in.
    pub(crate) fn new(monotonic: bool) -> Self {
        Sum {
            value_map: ValueMap::new(),
            monotonic,
            start: Mutex::new(SystemTime::now()),
        }
    }

    pub(crate) fn measure(&self, measurement: T, attrs: AttributeSet) {
        self.value_map.measure(measurement, attrs)
    }

    pub(crate) fn delta(
        &self,
        dest: Option<&mut dyn Aggregation>,
    ) -> (usize, Option<Box<dyn Aggregation>>) {
        let t = SystemTime::now();

        let s_data = dest.and_then(|d| d.as_mut().downcast_mut::<data::Sum<T>>());
        let mut new_agg = if s_data.is_none() {
            Some(data::Sum {
                data_points: vec![],
                temporality: Temporality::Delta,
                is_monotonic: self.monotonic,
            })
        } else {
            None
        };
        let s_data = s_data.unwrap_or_else(|| new_agg.as_mut().expect("present if s_data is none"));
        s_data.temporality = Temporality::Delta;
        s_data.is_monotonic = self.monotonic;
        s_data.data_points.clear();

        let expected_capacity = self.value_map.values.len() + 1;

        if expected_capacity > s_data.data_points.capacity() {
            s_data
                .data_points
                .reserve_exact(expected_capacity - s_data.data_points.capacity());
        }

        let prev_start = self.start.lock().map(|start| *start).unwrap_or(t);
        if self
            .value_map
            .has_no_value_attribute_value
            .swap(false, Ordering::AcqRel)
        {
            s_data.data_points.push(DataPoint {
                attributes: AttributeSet::default(),
                start_time: Some(prev_start),
                time: Some(t),
                value: self.value_map.no_attribute_value.get_and_reset_value(),
                exemplars: vec![],
            });
        }

        self.value_map.values.iter().for_each(|entry| {
            let attrs = entry.key().clone(); // Clone the key to use it for removal
            let value = entry.value().clone(); // Assuming value can be cloned. Adjust as necessary for your data structure

            // Now, push the data into s_data.data_points as before
            s_data.data_points.push(DataPoint {
                attributes: attrs,
                start_time: Some(prev_start),
                time: Some(t),
                value,
                exemplars: vec![],
            });

            // After processing, remove the entry from DashMap
            self.value_map.values.remove(entry.key());
        });

        // The delta collection cycle resets.
        if let Ok(mut start) = self.start.lock() {
            *start = t;
        }

        (
            s_data.data_points.len(),
            new_agg.map(|a| Box::new(a) as Box<_>),
        )
    }

    pub(crate) fn cumulative(
        &self,
        dest: Option<&mut dyn Aggregation>,
    ) -> (usize, Option<Box<dyn Aggregation>>) {
        let t = SystemTime::now();

        let s_data = dest.and_then(|d| d.as_mut().downcast_mut::<data::Sum<T>>());
        let mut new_agg = if s_data.is_none() {
            Some(data::Sum {
                data_points: vec![],
                temporality: Temporality::Cumulative,
                is_monotonic: self.monotonic,
            })
        } else {
            None
        };
        let s_data = s_data.unwrap_or_else(|| new_agg.as_mut().expect("present if s_data is none"));
        s_data.temporality = Temporality::Cumulative;
        s_data.is_monotonic = self.monotonic;
        s_data.data_points.clear();

        let n = self.value_map.values.len() + 1;

        if n > s_data.data_points.capacity() {
            s_data
                .data_points
                .reserve_exact(n - s_data.data_points.capacity());
        }

        let prev_start = self.start.lock().map(|start| *start).unwrap_or(t);

        if self
            .value_map
            .has_no_value_attribute_value
            .load(Ordering::Acquire)
        {
            s_data.data_points.push(DataPoint {
                attributes: AttributeSet::default(),
                start_time: Some(prev_start),
                time: Some(t),
                value: self.value_map.no_attribute_value.get_value(),
                exemplars: vec![],
            });
        }

        // TODO: This will use an unbounded amount of memory if there
        // are unbounded number of attribute sets being aggregated. Attribute
        // sets that become "stale" need to be forgotten so this will not
        // overload the system.
        self.value_map.values.iter().for_each(|entry| {
            let attrs = entry.key().clone(); // Assuming AttributeSet can be cloned
            let value = *entry.value();
            s_data.data_points.push(DataPoint {
                attributes: attrs,
                start_time: Some(prev_start), // Ensure prev_start is obtained correctly
                time: Some(t),
                value,
                exemplars: vec![],
            });
        });

        (
            s_data.data_points.len(),
            new_agg.map(|a| Box::new(a) as Box<_>),
        )
    }
}

/// Summarizes a set of pre-computed sums as their arithmetic sum.
pub(crate) struct PrecomputedSum<T: Number<T>> {
    value_map: ValueMap<T>,
    monotonic: bool,
    start: Mutex<SystemTime>,
    reported: Mutex<HashMap<AttributeSet, T>>,
}

impl<T: Number<T>> PrecomputedSum<T> {
    pub(crate) fn new(monotonic: bool) -> Self {
        PrecomputedSum {
            value_map: ValueMap::new(),
            monotonic,
            start: Mutex::new(SystemTime::now()),
            reported: Mutex::new(Default::default()),
        }
    }

    pub(crate) fn measure(&self, measurement: T, attrs: AttributeSet) {
        self.value_map.measure(measurement, attrs)
    }

    pub(crate) fn delta(
        &self,
        dest: Option<&mut dyn Aggregation>,
    ) -> (usize, Option<Box<dyn Aggregation>>) {
        let t = SystemTime::now();
        let prev_start = self.start.lock().map(|start| *start).unwrap_or(t);

        let s_data = dest.and_then(|d| d.as_mut().downcast_mut::<data::Sum<T>>());
        let mut new_agg = if s_data.is_none() {
            Some(data::Sum {
                data_points: vec![],
                temporality: Temporality::Delta,
                is_monotonic: self.monotonic,
            })
        } else {
            None
        };
        let s_data = s_data.unwrap_or_else(|| new_agg.as_mut().expect("present if s_data is none"));
        s_data.data_points.clear();
        s_data.temporality = Temporality::Delta;
        s_data.is_monotonic = self.monotonic;

        let expected_capacity = self.value_map.values.len() + 1;

        if expected_capacity > s_data.data_points.capacity() {
            s_data
                .data_points
                .reserve_exact(expected_capacity - s_data.data_points.capacity());
        }
        let mut new_reported = HashMap::with_capacity(expected_capacity);
        let mut reported = match self.reported.lock() {
            Ok(r) => r,
            Err(_) => return (0, None),
        };

        if self
            .value_map
            .has_no_value_attribute_value
            .swap(false, Ordering::AcqRel)
        {
            s_data.data_points.push(DataPoint {
                attributes: AttributeSet::default(),
                start_time: Some(prev_start),
                time: Some(t),
                value: self.value_map.no_attribute_value.get_and_reset_value(),
                exemplars: vec![],
            });
        }

        let default = T::default();
        self.value_map.values.iter().for_each(|entry| {
            let attrs = entry.key();
            let value = *entry.value();
            let delta = value - *reported.get(attrs).unwrap_or(&default);
            if delta != default {
                new_reported.insert(attrs.clone(), value);
            }

            s_data.data_points.push(DataPoint {
                attributes: attrs.clone(), // Assuming clone is implemented for AttributeSet
                start_time: Some(prev_start),
                time: Some(t),
                value: delta,
                exemplars: vec![],
            });
        });

        // The delta collection cycle resets.
        if let Ok(mut start) = self.start.lock() {
            *start = t;
        }

        *reported = new_reported;
        drop(reported); // drop before values guard is dropped

        (
            s_data.data_points.len(),
            new_agg.map(|a| Box::new(a) as Box<_>),
        )
    }

    pub(crate) fn cumulative(
        &self,
        dest: Option<&mut dyn Aggregation>,
    ) -> (usize, Option<Box<dyn Aggregation>>) {
        let t = SystemTime::now();
        let prev_start = self.start.lock().map(|start| *start).unwrap_or(t);

        let s_data = dest.and_then(|d| d.as_mut().downcast_mut::<data::Sum<T>>());
        let mut new_agg = if s_data.is_none() {
            Some(data::Sum {
                data_points: vec![],
                temporality: Temporality::Cumulative,
                is_monotonic: self.monotonic,
            })
        } else {
            None
        };
        let s_data = s_data.unwrap_or_else(|| new_agg.as_mut().expect("present if s_data is none"));
        s_data.data_points.clear();
        s_data.temporality = Temporality::Cumulative;
        s_data.is_monotonic = self.monotonic;

        let expected_capacity = self.value_map.values.len() + 1;

        if expected_capacity > s_data.data_points.capacity() {
            s_data
                .data_points
                .reserve_exact(expected_capacity - s_data.data_points.capacity());
        }

        let mut new_reported = HashMap::with_capacity(expected_capacity);
        let mut reported = match self.reported.lock() {
            Ok(r) => r,
            Err(_) => return (0, None),
        };

        if self
            .value_map
            .has_no_value_attribute_value
            .load(Ordering::Acquire)
        {
            s_data.data_points.push(DataPoint {
                attributes: AttributeSet::default(),
                start_time: Some(prev_start),
                time: Some(t),
                value: self.value_map.no_attribute_value.get_value(),
                exemplars: vec![],
            });
        }

        let default = T::default();
        self.value_map.values.iter().for_each(|entry| {
            let attrs = entry.key().clone();
            let value = *entry.value();
            let delta = value - *reported.get(&attrs).unwrap_or(&default);
            if delta != default {
                new_reported.insert(attrs.clone(), value);
            }
            s_data.data_points.push(DataPoint {
                attributes: attrs.clone(),
                start_time: Some(prev_start),
                time: Some(t),
                value: delta,
                exemplars: vec![],
            });
        });

        *reported = new_reported;
        drop(reported); // drop before values guard is dropped

        (
            s_data.data_points.len(),
            new_agg.map(|a| Box::new(a) as Box<_>),
        )
    }
}
