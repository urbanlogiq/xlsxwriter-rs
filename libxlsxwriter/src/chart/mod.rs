mod constants;
mod series;
mod structs;

pub use self::constants::*;
pub use self::series::*;
pub use self::structs::*;
use super::{convert_str, Workbook};
use std::os::raw::c_char;

/// The Chart object represents an Excel chart. It provides functions for adding data series to the chart and for configuring the chart.
///
/// A Chart object isn't created directly. Instead a chart is created by calling the Workbook.add_chart() function from a Workbook object. For example:
/// ```rust
/// use xlsxwriter::*;
/// # fn main() -> Result<(), XlsxError> {
/// let workbook = Workbook::new("test-chart.xlsx");
/// let mut worksheet = workbook.add_worksheet(None)?;
/// write_worksheet(&mut worksheet)?; // write worksheet contents
/// let mut chart = workbook.add_chart(ChartType::Column);
/// chart.add_series(None, Some("=Sheet1!$A$1:$A$5"));
/// chart.add_series(None, Some("=Sheet1!$B$1:$B$5"));
/// chart.add_series(None, Some("=Sheet1!$C$1:$C$5"));
/// worksheet.insert_chart(1, 3, &chart)?;
/// workbook.close()
/// # }
/// # fn write_worksheet(worksheet: &mut Worksheet) -> Result<(), XlsxError> {
/// # for i in 0..5 {
/// #     worksheet.write_number(i, 0, (i*10).into(), None)?;
/// #     worksheet.write_number(i, 1, (i*10 + 2).into(), None)?;
/// #     worksheet.write_number(i, 2, (i*10 + 4).into(), None)?;
/// # }
/// # Ok(())
/// # }
/// ```
/// The chart in the worksheet will look like this:
/// ![Result Image](https://github.com/informationsea/xlsxwriter-rs/raw/master/images/test-chart-1.png)
///
///
/// The basic procedure for adding a chart to a worksheet is:
///
/// Create the chart with Workbook.add_chart().
/// Add one or more data series to the chart which refers to data in the workbook using Chart.add_series().
/// Configure the chart with the other available functions shown below.
/// Insert the chart into a worksheet using Worksheet.insert_chart().
pub struct Chart<'a> {
    pub(crate) _workbook: &'a Workbook,
    pub(crate) chart: *mut libxlsxwriter_sys::lxw_chart,
}

impl<'a> Chart<'a> {
    /// In Excel a chart **series** is a collection of information that defines which data is plotted such as the categories and values. It is also used to define the formatting for the data.
    ///
    /// For an libxlsxwriter chart object the chart_add_series() function is used to set the categories and values of the series:
    /// ```rust
    /// # use xlsxwriter::*;
    /// # fn main() -> Result<(), XlsxError> {
    /// # let workbook = Workbook::new("test-chart-add_series-1.xlsx");
    /// # let mut worksheet = workbook.add_worksheet(None)?;
    /// # write_worksheet(&mut worksheet)?; // write worksheet contents
    /// # let mut chart = workbook.add_chart(ChartType::Column);
    /// chart.add_series(Some("=Sheet1!$A$1:$A$5"), Some("=Sheet1!$B$1:$B$5"));
    /// # worksheet.insert_chart(1, 3, &chart)?;
    /// # workbook.close()
    /// # }
    /// # fn write_worksheet(worksheet: &mut Worksheet) -> Result<(), XlsxError> {
    /// # for i in 0..5 {
    /// #     worksheet.write_string(i, 0, &format!("value {}", i + 1), None)?;
    /// #     worksheet.write_number(i, 1, (i*10 + 2).into(), None)?;
    /// # }
    /// # Ok(())
    /// # }
    /// ```
    /// The series parameters are:
    ///
    /// *categories: This sets the chart category labels. The category is more or less the same as the X axis. In most Excel chart types the categories property is optional and the chart will just assume a sequential series from 1..n:
    /// ```rust
    /// # use xlsxwriter::*;
    /// # fn main() -> Result<(), XlsxError> {
    /// # let workbook = Workbook::new("test-chart-add_series-2.xlsx");
    /// # let mut worksheet = workbook.add_worksheet(None)?;
    /// # write_worksheet(&mut worksheet)?; // write worksheet contents
    /// # let mut chart = workbook.add_chart(ChartType::Column);
    /// chart.add_series(None, Some("=Sheet1!$B$1:$B$5"));
    /// # worksheet.insert_chart(1, 3, &chart)?;
    /// # workbook.close()
    /// # }
    /// # fn write_worksheet(worksheet: &mut Worksheet) -> Result<(), XlsxError> {
    /// # for i in 0..5 {
    /// #     worksheet.write_string(i, 0, &format!("value {}", i + 1), None)?;
    /// #     worksheet.write_number(i, 1, (i*10 + 2).into(), None)?;
    /// # }
    /// # Ok(())
    /// # }
    /// ```
    /// * values: This is the most important property of a series and is the only mandatory option for every chart object. This parameter links the chart with the worksheet data that it displays.
    ///
    /// The categories and values should be a string formula like "=Sheet1!$A$2:$A$7" in the same way it is represented in Excel. This is convenient when recreating a chart from an example in Excel but it is trickier to generate programmatically. For these cases you can set the categories and values to None and use the ChartSeries.set_categories() and ChartSeries.set_values() functions:
    /// ```rust
    /// # use xlsxwriter::*;
    /// # fn main() -> Result<(), XlsxError> {
    /// # let workbook = Workbook::new("test-chart-add_series-3.xlsx");
    /// # let mut worksheet = workbook.add_worksheet(None)?;
    /// # write_worksheet(&mut worksheet)?; // write worksheet contents
    /// # let mut chart = workbook.add_chart(ChartType::Column);
    /// let mut series = chart.add_series(None, None);
    /// series.set_categories("Sheet1", 0, 0, 4, 0); // "=Sheet1!$A$1:$A$5"
    /// series.set_values("Sheet1", 0, 1, 4, 1);     // "=Sheet1!$B$1:$B$5"
    /// # worksheet.insert_chart(1, 3, &chart)?;
    /// # workbook.close()
    /// # }
    /// # fn write_worksheet(worksheet: &mut Worksheet) -> Result<(), XlsxError> {
    /// # for i in 0..5 {
    /// #     worksheet.write_string(i, 0, &format!("value {}", i + 1), None)?;
    /// #     worksheet.write_number(i, 1, (i*10 + 2).into(), None)?;
    /// # }
    /// # Ok(())
    /// # }
    /// ```
    /// As shown in the previous example the return value from Chart.add_series() is a `ChartSeries` struct. This can be used in other functions that configure a series.
    ///
    /// More than one series can be added to a chart. The series numbering and order in the Excel chart will be the same as the order in which they are added in libxlsxwriter:
    ///
    /// ```rust
    /// # use xlsxwriter::*;
    /// # fn main() -> Result<(), XlsxError> {
    /// # let workbook = Workbook::new("test-chart-add_series-4.xlsx");
    /// # let mut worksheet = workbook.add_worksheet(None)?;
    /// # write_worksheet(&mut worksheet)?; // write worksheet contents
    /// # let mut chart = workbook.add_chart(ChartType::Column);
    /// chart.add_series(None, Some("=Sheet1!$A$1:$A$5"));
    /// chart.add_series(None, Some("=Sheet1!$B$1:$B$5"));
    /// chart.add_series(None, Some("=Sheet1!$C$1:$C$5"));
    /// # worksheet.insert_chart(1, 3, &chart)?;
    /// # workbook.close()
    /// # }
    /// # fn write_worksheet(worksheet: &mut Worksheet) -> Result<(), XlsxError> {
    /// # for i in 0..5 {
    /// #     worksheet.write_number(i, 0, (i*10 + 1).into(), None)?;
    /// #     worksheet.write_number(i, 1, (i*10 + 2).into(), None)?;
    /// #     worksheet.write_number(i, 2, (i*10 + 5).into(), None)?;
    /// # }
    /// # Ok(())
    /// # }
    /// ```
    /// It is also possible to specify non-contiguous ranges:
    /// ```rust
    /// # use xlsxwriter::*;
    /// # fn main() -> Result<(), XlsxError> {
    /// # let workbook = Workbook::new("test-chart-add_series-5.xlsx");
    /// # let mut worksheet = workbook.add_worksheet(None)?;
    /// # write_worksheet(&mut worksheet)?; // write worksheet contents
    /// # let mut chart = workbook.add_chart(ChartType::Column);
    /// chart.add_series(Some("=(Sheet1!$A$1:$A$5,Sheet1!$A$10:$A$18)"), Some("=(Sheet1!$B$1:$B$5,Sheet1!$B$10:$B$18)"));
    /// # worksheet.insert_chart(1, 3, &chart)?;
    /// # workbook.close()
    /// # }
    /// # fn write_worksheet(worksheet: &mut Worksheet) -> Result<(), XlsxError> {
    /// # for i in 0..20 {
    /// #     worksheet.write_string(i, 0, &format!("value {}", i + 1), None)?;
    /// #     worksheet.write_number(i, 1, (i*10 + 2).into(), None)?;
    /// # }
    /// # Ok(())
    /// # }
    /// ```

    pub fn add_series(
        &mut self,
        categories: Option<&str>,
        values: Option<&str>,
    ) -> ChartSeries<'a> {
        let categories_vec = categories.map(convert_str);
        let values_vec = values.map(convert_str);
        let mut const_str = self._workbook.const_str.borrow_mut();
        let series = unsafe {
            libxlsxwriter_sys::chart_add_series(
                self.chart,
                categories_vec
                    .as_ref()
                    .map(|x| x.as_ptr())
                    .unwrap_or(std::ptr::null()) as *const c_char,
                values_vec
                    .as_ref()
                    .map(|x| x.as_ptr())
                    .unwrap_or(std::ptr::null()) as *const c_char,
            )
        };
        if let Some(x) = categories_vec {
            const_str.push(x);
        }
        if let Some(x) = values_vec {
            const_str.push(x);
        }
        ChartSeries {
            _workbook: self._workbook,
            chart_series: series,
        }
    }
}
