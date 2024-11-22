pub(crate) const DEFAULT_MAX_NUMBER_OF_ATTRIBUTES: u32 = 128;
pub(crate) const DEFAULT_MAX_ATTRIBUTE_VALUE_LENGTH: u32 = 1024;

/// Log limit configuration to keep attributes and their values in a reasonable size.
#[derive(Copy, Clone, Debug)]
pub struct LogLimits {
    /// The maximum number of attributes that can be added to a log record.
    max_number_of_attributes: u32,
    /// The maximum length allowed for an attribute value.
    max_attribute_value_length: u32,
}

impl Default for LogLimits {
    fn default() -> Self {
        LogLimits {
            max_number_of_attributes: DEFAULT_MAX_NUMBER_OF_ATTRIBUTES,
            max_attribute_value_length: DEFAULT_MAX_ATTRIBUTE_VALUE_LENGTH,
        }
    }
}

impl LogLimits {
    /// Creates a new `LogLimits` instance with default values.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the maximum number of attributes allowed per log record.
    pub fn with_max_attributes(mut self, max_attributes: u32) -> Self {
        self.max_number_of_attributes = max_attributes;
        self
    }

    /// Sets the maximum length allowed for attribute values.
    pub fn with_max_value_length(mut self, max_length: u32) -> Self {
        self.max_attribute_value_length = max_length;
        self
    }

    /// Returns the maximum number of attributes allowed per log record.
    #[inline]
    pub fn max_number_of_attributes(&self) -> u32 {
        self.max_number_of_attributes
    }

    /// Returns the maximum length allowed for attribute values.
    #[inline]
    pub fn max_attribute_value_length(&self) -> u32 {
        self.max_attribute_value_length
    }
}