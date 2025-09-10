import React from 'react';
import {
  LocalizationProvider,
  DateRangePicker as MuiDateRangePicker,
  DateRange,
} from '@mui/x-date-pickers-pro';
import { AdapterDateFns } from '@mui/x-date-pickers/AdapterDateFns';
import { TextField, Box } from '@mui/material';
import { DateRangePickerProps } from '@mui/x-date-pickers-pro/DateRangePicker';

type DateRangeValue = DateRange<Date>;

interface CustomDateRangePickerProps
  extends Omit<DateRangePickerProps<Date>, 'value' | 'onChange' | 'renderInput'> {
  value: DateRangeValue;
  onChange: (newValue: DateRangeValue) => void;
}

const DateRangePicker: React.FC<CustomDateRangePickerProps> = ({
  value,
  onChange,
  ...props
}) => {
  return (
    <LocalizationProvider dateAdapter={AdapterDateFns}>
      <MuiDateRangePicker
        value={value}
        onChange={onChange}
        renderInput={(startProps, endProps) => (
          <>
            <TextField {...startProps} size="small" />
            <Box sx={{ mx: 2 }}> to </Box>
            <TextField {...endProps} size="small" />
          </>
        )}
        {...props}
      />
    </LocalizationProvider>
  );
};

export default DateRangePicker;
